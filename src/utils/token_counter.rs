// src/utils/token_counter.rs
#[derive(Debug)] // Add this derive
pub struct TokenCounter {
    // Qwen's context length is 32,768 tokens [3]
    max_tokens: usize,
}

impl TokenCounter {
    pub fn new(max_tokens: usize) -> Self {
        TokenCounter { max_tokens }
    }

    // Simple estimation: ~4 chars per token on average
    pub fn estimate_tokens(&self, text: &str) -> usize {
        text.len() / 4
    }

    pub fn would_exceed_limit(&self, current_total: usize, new_text: &str) -> bool {
        current_total + self.estimate_tokens(new_text) > self.max_tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_estimation() {
        let counter = TokenCounter::new(100);
        assert_eq!(counter.estimate_tokens("test"), 1); // 4 chars = ~1 token
        assert_eq!(counter.estimate_tokens("longer text here"), 4); // 16 chars = ~4 tokens
    }

    #[test]
    fn test_limit_check() {
        let counter = TokenCounter::new(10);
        assert!(!counter.would_exceed_limit(5, "test")); // 5 + 1 = 6 tokens
        assert!(counter.would_exceed_limit(9, "long text")); // 9 + 2 > 10 tokens
    }

    #[test]
    fn test_debug_output() {
        let counter = TokenCounter::new(100);
        let debug_str = format!("{:?}", counter);
        assert!(debug_str.contains("100"));
    }
}
