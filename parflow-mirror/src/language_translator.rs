use semantic_compiler::PatternType;

pub struct LanguageTranslator;

impl LanguageTranslator {
    pub fn translate_pattern(&self, pattern: PatternType, source_lang: &str, target_lang: &str) -> String {
        // Use pattern by reference in the match to avoid moving it
        match (&pattern, source_lang, target_lang) {
            (PatternType::FibonacciLike, "python", "rust") => {
                r#"
// Optimized Rust version of Fibonacci
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=n {
                let next = a + b;
                a = b;
                b = next;
            }
            b
        }
    }
}
"#.to_string()
            }
            (PatternType::MapReduce, "javascript", "python") => {
                r#"
# Python optimized map-reduce pattern
def process_data(data):
    # Map phase
    mapped = [x * 2 for x in data]
    # Reduce phase  
    result = sum(mapped)
    return result
"#.to_string()
            }
            _ => format!("// Pattern {:?} translation from {} to {}", pattern, source_lang, target_lang),
        }
    }
}
