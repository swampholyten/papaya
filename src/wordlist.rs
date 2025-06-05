use rand::seq::SliceRandom;

static DEFAULT_WORDS: &[&str] = &[
    "apple", "banana", "orange", "grape", "mango",
    "computer", "keyboard", "monitor", "mouse", "screen",
    "rust", "cargo", "compile", "code", "debug",
    "terminal", "prompt", "library", "crate", "module",
];

pub fn random_words(count: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut words = DEFAULT_WORDS.to_vec();
    words.shuffle(&mut rng);
    words.iter().take(count).map(|w| w.to_string()).collect()
}
