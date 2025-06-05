use random_word::{self, Lang};

pub fn random_words(count: usize, lang: Lang) -> Vec<String> {
    (0..count).map(|_| random_word::get(lang).to_string()).collect()
}
