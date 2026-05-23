//! Stempel-based Polish stemmer using the Egothor trie algorithm.
//!
//! This implements the exact same algorithm as Lucene's StempelStemmer:
//! 1. Load a pre-compiled MultiTrie2 from `stemmer_20000.tbl`
//! 2. For each word, traverse the trie backwards (right-to-left) using `getLastOnPath`
//! 3. The trie returns a "patch command" string
//! 4. Apply the patch (Diff::apply) to transform the word into its stem
//!
//! The patch command language:
//! - Commands are pairs of (cmd_char, param_char), processed from end of word
//! - '-' + param: move cursor back by (param - 'a' + 1) positions
//! - 'R' + param: replace char at cursor with param
//! - 'D' + param: delete (param - 'a' + 1) chars starting at cursor
//! - 'I' + param: insert param after cursor
//!
//! The trie table (stemmer_20000.tbl) is compiled from 20,000 Polish word pairs
//! and encodes the complete morphological transformation rules for Polish.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

use pizza_engine::analysis::{Token, TokenFilter};

/// A cell in the Egothor trie row.
#[derive(Clone, Debug)]
struct Cell {
    cmd: i32,   // index into commands table (-1 = no command)
    _cnt: i32,  // usage count (not used at runtime)
    ref_: i32,  // index of next row (-1 = no transition)
    skip: i32,  // chars to skip during traversal
}

/// A row in the Egothor trie (maps char → Cell).
#[derive(Clone, Debug)]
struct Row {
    cells: Vec<(char, Cell)>,
}

impl Row {
    /// Load a row from binary data at the given offset.
    /// Returns (row, bytes_consumed).
    fn from_bytes(data: &[u8], offset: usize) -> (Self, usize) {
        let mut pos = offset;
        let count = read_i32(data, pos) as usize;
        pos += 4;

        let mut cells = Vec::with_capacity(count);
        for _ in 0..count {
            let ch = read_char(data, pos);
            pos += 2;
            let cmd = read_i32(data, pos);
            pos += 4;
            let _cnt = read_i32(data, pos);
            pos += 4;
            let ref_ = read_i32(data, pos);
            pos += 4;
            let skip = read_i32(data, pos);
            pos += 4;
            cells.push((ch, Cell { cmd, _cnt, ref_, skip }));
        }

        (Self { cells }, pos - offset)
    }

    /// Get the Cell for a given character.
    #[inline]
    fn at(&self, ch: char) -> Option<&Cell> {
        // Linear search is fine for small rows (avg ~4 entries)
        self.cells.iter().find(|(c, _)| *c == ch).map(|(_, cell)| cell)
    }

    /// Get the command index for a character.
    #[inline]
    fn get_cmd(&self, ch: char) -> i32 {
        self.at(ch).map_or(-1, |c| c.cmd)
    }

    /// Get the reference (next row index) for a character.
    #[inline]
    fn get_ref(&self, ch: char) -> i32 {
        self.at(ch).map_or(-1, |c| c.ref_)
    }
}

/// An Egothor trie (single trie in a MultiTrie).
#[derive(Clone, Debug)]
struct Trie {
    forward: bool,
    root: usize,
    cmds: Vec<String>,
    rows: Vec<Row>,
}

impl Trie {
    /// Load a trie from binary data.
    fn from_bytes(data: &[u8], offset: usize) -> (Self, usize) {
        let mut pos = offset;
        let forward = data[pos] != 0;
        pos += 1;
        let root = read_i32(data, pos) as usize;
        pos += 4;

        // Read commands
        let cmd_count = read_i32(data, pos) as usize;
        pos += 4;
        let mut cmds = Vec::with_capacity(cmd_count);
        for _ in 0..cmd_count {
            let (s, consumed) = read_utf(data, pos);
            cmds.push(s);
            pos += consumed;
        }

        // Read rows
        let row_count = read_i32(data, pos) as usize;
        pos += 4;
        let mut rows = Vec::with_capacity(row_count);
        for _ in 0..row_count {
            let (row, consumed) = Row::from_bytes(data, pos);
            rows.push(row);
            pos += consumed;
        }

        (Self { forward, root, cmds, rows }, pos - offset)
    }

    /// Get the last command on the path matching the key.
    /// This traverses the trie following the key characters, collecting
    /// the most recent command found along the way.
    fn get_last_on_path(&self, key: &[char]) -> Option<&str> {
        if self.rows.is_empty() || key.is_empty() {
            return None;
        }

        let mut now = &self.rows[self.root];
        let mut last: Option<&str> = None;

        let iter: Vec<char> = if self.forward {
            key.to_vec()
        } else {
            key.iter().rev().copied().collect()
        };

        let len = iter.len();
        for i in 0..len - 1 {
            let ch = iter[i];
            let w = now.get_cmd(ch);
            if w >= 0 {
                last = Some(&self.cmds[w as usize]);
            }
            let r = now.get_ref(ch);
            if r >= 0 && (r as usize) < self.rows.len() {
                now = &self.rows[r as usize];
            } else {
                return last;
            }
        }

        // Last character
        let w = now.get_cmd(iter[len - 1]);
        if w >= 0 {
            Some(&self.cmds[w as usize])
        } else {
            last
        }
    }
}

/// MultiTrie: a collection of sub-tries whose results are concatenated.
/// MultiTrie2 extends this with skip-command delimiting (handled in getLastOnPath).
#[derive(Clone, Debug)]
struct MultiTrie {
    forward: bool,
    _by: i32,
    tries: Vec<Trie>,
}

impl MultiTrie {
    /// Load from the binary stemmer table.
    fn from_bytes(data: &[u8]) -> Self {
        let mut pos = 0;

        // Read method string (e.g. "-0ME2")
        let (_method, consumed) = read_utf(data, pos);
        pos += consumed;

        // MultiTrie: forward, BY, trie count, tries
        let forward = data[pos] != 0;
        pos += 1;
        let by = read_i32(data, pos);
        pos += 4;
        let trie_count = read_i32(data, pos) as usize;
        pos += 4;

        let mut tries = Vec::with_capacity(trie_count);
        for _ in 0..trie_count {
            let (trie, consumed) = Trie::from_bytes(data, pos);
            tries.push(trie);
            pos += consumed;
        }

        Self {
            forward,
            _by: by,
            tries,
        }
    }

    /// Get the combined patch command for a word.
    /// MultiTrie2 concatenates results from each sub-trie, stopping at EOM ('*').
    fn get_last_on_path(&self, key: &[char]) -> Option<String> {
        let mut result = String::new();
        for trie in &self.tries {
            if let Some(r) = trie.get_last_on_path(key) {
                if r.len() == 1 && r.starts_with('*') {
                    // EOM marker - stop concatenating
                    return if result.is_empty() { None } else { Some(result) };
                }
                result.push_str(r);
            } else {
                // No match in this sub-trie
                return if result.is_empty() { None } else { Some(result) };
            }
        }
        if result.is_empty() { None } else { Some(result) }
    }
}

/// Apply a patch command to a word buffer (Diff::apply from Egothor).
///
/// The command is a sequence of (cmd_char, param_char) pairs applied from the
/// end of the word towards the beginning.
fn apply_diff(word: &mut Vec<char>, diff: &str) {
    if diff.is_empty() {
        return;
    }

    let diff_chars: Vec<char> = diff.chars().collect();
    let mut pos = word.len() as i32 - 1;
    if pos < 0 {
        return;
    }

    let mut i = 0;
    while i + 1 < diff_chars.len() {
        let cmd = diff_chars[i];
        let param = diff_chars[i + 1];
        let par_num = (param as i32) - ('a' as i32) + 1;

        match cmd {
            '-' => {
                // Move cursor back
                pos = pos - par_num + 1;
            }
            'R' => {
                // Replace character at cursor
                if pos >= 0 && (pos as usize) < word.len() {
                    word[pos as usize] = param;
                }
            }
            'D' => {
                // Delete par_num chars ending at pos
                let end = pos as usize;
                let start = (pos - par_num + 1) as usize;
                if start <= end && end < word.len() {
                    word.drain(start..=end);
                    pos = start as i32 - 1;
                    // After deletion, pos already decremented below
                    i += 2;
                    continue; // skip the pos-- at the end
                }
            }
            'I' => {
                // Insert param after cursor
                pos += 1;
                if pos >= 0 && (pos as usize) <= word.len() {
                    word.insert(pos as usize, param);
                }
            }
            _ => {}
        }

        pos -= 1;
        i += 2;
    }
}

/// The Stempel stemmer table, loaded from `stemmer_20000.tbl`.
static STEMMER_TABLE: &[u8] = include_bytes!("../data/stemmer_20000.tbl");

/// Polish stemming filter using the Stempel algorithm with the real Egothor trie.
///
/// This is a faithful port of Lucene's StempelStemmer that uses the actual
/// `stemmer_20000.tbl` data (compiled from 20,000 Polish word pairs).
///
/// Equivalent to Elasticsearch's `stemmer` filter with `language: "polish"`.
#[derive(Clone)]
pub struct StempelStemFilter {
    trie: MultiTrie,
}

impl core::fmt::Debug for StempelStemFilter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StempelStemFilter").finish()
    }
}

impl Default for StempelStemFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl StempelStemFilter {
    /// Create a new Stempel stemmer by loading the embedded trie table.
    pub fn new() -> Self {
        let trie = MultiTrie::from_bytes(STEMMER_TABLE);
        Self { trie }
    }

    /// Stem a Polish word using the Egothor trie + diff patch.
    ///
    /// Returns the stemmed form, or None if no transformation applies
    /// or the result would be empty.
    pub fn stem(&self, word: &str) -> Option<String> {
        let lower = word.to_lowercase();
        let chars: Vec<char> = lower.chars().collect();

        if chars.len() < 3 {
            return None;
        }

        // Look up the patch command in the MultiTrie
        let cmd = self.trie.get_last_on_path(&chars)?;

        // Apply the patch
        let mut buffer = chars;
        apply_diff(&mut buffer, &cmd);

        if buffer.is_empty() {
            return None;
        }

        let result: String = buffer.into_iter().collect();
        if result == lower {
            return None; // No change
        }

        Some(result)
    }
}

impl TokenFilter for StempelStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if let Some(stemmed) = self.stem(term) {
            if stemmed != term {
                token.term = Cow::Owned(stemmed);
            }
        }
        (false, None)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Binary deserialization helpers (Java DataInputStream compatible)
// ═══════════════════════════════════════════════════════════════════════════════

/// Read a big-endian i32.
#[inline]
fn read_i32(data: &[u8], pos: usize) -> i32 {
    i32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]])
}

/// Read a big-endian Java char (UTF-16 code unit, 2 bytes).
#[inline]
fn read_char(data: &[u8], pos: usize) -> char {
    let code = u16::from_be_bytes([data[pos], data[pos + 1]]);
    char::from_u32(code as u32).unwrap_or('\u{FFFD}')
}

/// Read a Java modified UTF-8 string (DataInput.readUTF format).
/// Returns (string, total_bytes_consumed including length prefix).
fn read_utf(data: &[u8], pos: usize) -> (String, usize) {
    let len = u16::from_be_bytes([data[pos], data[pos + 1]]) as usize;
    let start = pos + 2;
    let end = start + len;

    // Java modified UTF-8 decoding
    let mut chars = Vec::new();
    let mut i = start;
    while i < end {
        let b1 = data[i];
        if b1 < 0x80 {
            // Single byte: 0xxxxxxx
            chars.push(b1 as char);
            i += 1;
        } else if b1 & 0xE0 == 0xC0 {
            // Two bytes: 110xxxxx 10xxxxxx
            if i + 1 < end {
                let b2 = data[i + 1];
                let code = ((b1 as u32 & 0x1F) << 6) | (b2 as u32 & 0x3F);
                chars.push(char::from_u32(code).unwrap_or('\u{FFFD}'));
            }
            i += 2;
        } else if b1 & 0xF0 == 0xE0 {
            // Three bytes: 1110xxxx 10xxxxxx 10xxxxxx
            if i + 2 < end {
                let b2 = data[i + 1];
                let b3 = data[i + 2];
                let code = ((b1 as u32 & 0x0F) << 12)
                    | ((b2 as u32 & 0x3F) << 6)
                    | (b3 as u32 & 0x3F);
                chars.push(char::from_u32(code).unwrap_or('\u{FFFD}'));
            }
            i += 3;
        } else {
            i += 1; // Skip unknown
        }
    }

    (chars.into_iter().collect(), len + 2)
}
