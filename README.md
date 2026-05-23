# pizza-analysis-stempel

Polish stemming for the [Pizza](https://pizza.rs) search engine. Implements the Stempel algorithm using the Egothor trie structure, equivalent to Apache Lucene's `StempelStemmer`.

## Components

| Name | Type | Description |
|------|------|-------------|
| `stempel_stem` | Token Filter | Polish stemming via Stempel/Egothor trie algorithm |
| `polish_stop` | Token Filter | Polish stop words removal (186 words) |

## Usage

### Stempel Stemmer

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["lowercase", "polish_stop", "stempel_stem"]
  }
}
```

### Examples

| Input | Stemmed |
|-------|---------|
| `komputerów` | `komputer` |
| `programowania` | `programować` |
| `polskich` | `polski` |
| `książkami` | `książka` |

## Algorithm

The Stempel stemmer uses an Egothor MultiTrie2 structure:

1. **Trie lookup** — The input word is reversed and traversed through multiple sub-tries (backward/suffix-based matching)
2. **Patch command retrieval** — Each sub-trie returns a "patch command" string
3. **Diff application** — The patch commands are applied to the original word using operations:
   - `-` (seek forward in word)
   - `R` (replace character)
   - `D` (delete character)
   - `I` (insert character)
4. **Multi-trie combination** — Results from 8 sub-tries are combined, with each subsequent trie refining the result

This approach captures morphological transformations more accurately than simple suffix stripping, handling phenomena like vowel alternation (ó→o) and consonant mutation.

## Data Sources

- **Stemmer table**: `stemmer_20000.tbl` — Pre-compiled Egothor MultiTrie2 binary table from Apache Lucene (trained on 20,000 word forms)
- **Stop words**: 186 Polish stop words from Apache Lucene's `PolishAnalyzer`
- **License**: Apache 2.0 (from Apache Lucene)

## Technical Details

- Binary trie data is embedded via `include_bytes!` (~2.2MB)
- Trie deserialization uses Java `DataInputStream`-compatible reading (big-endian)
- The MultiTrie2 contains 8 sub-tries with 6,815 total rows and 664 commands
- Operates on `no_std` with alloc — suitable for embedded/WASM targets

## Features

- `embed-table` (default) — Embeds the stemmer table at compile time

## Comparison with Snowball Polish Stemmer

| Feature | Stempel | Snowball Polish |
|---------|---------|-----------------|
| Approach | Trained trie (data-driven) | Hand-written rules |
| Accuracy | Higher (trained on 20K forms) | Lower (simplified rules) |
| Binary size | ~2.2MB (trie table) | ~5KB (algorithm code) |
| Speed | Fast (trie lookup) | Faster (direct code) |
| Handles irregulars | Yes (learned) | Limited |

## License

Apache-2.0
