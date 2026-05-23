//! Polish stop words.

use hashbrown::HashSet;

use pizza_engine::analysis::{Token, TokenFilter};

/// Default Polish stop words.
pub const POLISH_STOP_WORDS: &[&str] = &[
    "a", "aby", "ach", "acz", "ale", "aż", "bo", "bowiem", "by", "byli",
    "bym", "być", "był", "była", "było", "były", "będzie", "będą",
    "cali", "cała", "cały", "ci", "cię", "co", "coraz", "czy", "czyli",
    "dla", "do", "gdy", "gdyby", "gdyż", "gdzie", "go", "i", "ich",
    "ile", "im", "inna", "inne", "inni", "inny", "innych", "iż",
    "ja", "jak", "jakaś", "jakichś", "jakie", "jakiś", "już",
    "każdy", "kiedy", "kilka", "kto", "która", "które", "którego",
    "której", "który", "których", "którym", "którzy", "ku",
    "lecz", "lub", "ma", "mają", "mi", "mimo", "między", "mnie",
    "mogą", "moim", "może", "można", "mu", "musi", "my", "mój",
    "na", "nad", "nam", "nas", "nasi", "nasz", "nawet", "nic",
    "nich", "nie", "niej", "nim", "niż", "no", "nsi",
    "o", "ob", "od", "ok", "obok", "on", "ona", "one", "oni", "ono",
    "oraz", "owszem", "pan", "pana", "pani", "po", "pod", "podczas",
    "pomimo", "ponad", "ponieważ", "powinien", "powinna", "powinni",
    "powinno", "poza", "przed", "przede", "przez", "przy",
    "roku", "również", "sam", "sama", "się", "skąd", "sobie",
    "sobą", "sposób", "swoje", "są", "ta", "tak", "taka", "taki",
    "takie", "tam", "te", "tego", "tej", "ten", "też", "to",
    "tobie", "trzeba", "tu", "tutaj", "twoi", "twoja", "twoje",
    "twój", "twym", "ty", "tych", "tylko", "tym", "tę", "u",
    "w", "wam", "was", "wasi", "wasz", "wasza", "wasze", "we",
    "więc", "wszystkich", "wszystkie", "wszystkim", "wszystko",
    "wtedy", "wy", "właśnie", "z", "za", "zapewne", "zawsze",
    "ze", "zeznowu", "znowu", "znów", "został", "żaden", "żadna",
    "żadne", "żadnych", "że", "żeby",
];

/// Polish stop word filter.
#[derive(Clone)]
pub struct PolishStopFilter {
    stop_words: HashSet<String>,
}

impl PolishStopFilter {
    pub fn new() -> Self {
        Self {
            stop_words: POLISH_STOP_WORDS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }

    pub fn with_words(words: Vec<String>) -> Self {
        Self {
            stop_words: words.into_iter().collect(),
        }
    }
}

impl Default for PolishStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenFilter for PolishStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let deleted = self.stop_words.contains(token.term.as_ref());
        (deleted, None)
    }
}
