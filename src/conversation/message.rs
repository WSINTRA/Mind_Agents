// src/conversation/message.rs
#[derive(Debug, Clone)]
pub struct Message {
    pub content: String,
    pub author: String,
}

impl Message {
    pub fn new(content: &str, author: &str) -> Self {
        Message {
            content: content.to_string(),
            author: author.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = Message::new("Test content", "Test author");
        assert_eq!(msg.content, "Test content");
        assert_eq!(msg.author, "Test author");
    }

    #[test]
    fn test_message_clone() {
        let original = Message::new("Test content", "Test author");
        let cloned = original.clone();

        assert_eq!(original.content, cloned.content);
        assert_eq!(original.author, cloned.author);
    }

    #[test]
    fn test_message_debug_output() {
        let msg = Message::new("Test content", "Test author");
        let debug_output = format!("{:?}", msg);

        assert!(debug_output.contains("Test content"));
        assert!(debug_output.contains("Test author"));
    }
}
