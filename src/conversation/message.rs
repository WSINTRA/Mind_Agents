// src/conversation/message.rs
#[derive(Debug, Clone)]
pub struct Message {
    pub content: String,
    pub author: String,
    pub is_valid: bool,
    pub visible: bool, // New field for output control
}

impl Message {
    pub fn new(content: &str, author: &str) -> Self {
        Message {
            content: content.to_string(),
            author: author.to_string(),
            is_valid: true,
            visible: true, // Default to visible
        }
    }

    pub fn with_visibility(content: &str, author: &str, visible: bool) -> Self {
        Message {
            content: content.to_string(),
            author: author.to_string(),
            is_valid: true,
            visible,
        }
    }

    pub fn invalidate(&mut self) {
        self.is_valid = false;
    }

    pub fn set_visibility(&mut self, visible: bool) {
        self.visible = visible;
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
        assert!(msg.is_valid);
        assert!(msg.visible);
    }

    #[test]
    fn test_message_with_visibility() {
        let msg = Message::with_visibility("Hidden content", "System", false);
        assert_eq!(msg.content, "Hidden content");
        assert_eq!(msg.author, "System");
        assert!(msg.is_valid);
        assert!(!msg.visible);
    }

    #[test]
    fn test_message_invalidation() {
        let mut msg = Message::new("Test content", "Test author");
        assert!(msg.is_valid);
        msg.invalidate();
        assert!(!msg.is_valid);
    }

    #[test]
    fn test_message_visibility_toggle() {
        let mut msg = Message::new("Test content", "Test author");
        assert!(msg.visible);
        msg.set_visibility(false);
        assert!(!msg.visible);
        msg.set_visibility(true);
        assert!(msg.visible);
    }

    #[test]
    fn test_message_clone() {
        let original = Message::new("Test content", "Test author");
        let cloned = original.clone();

        assert_eq!(original.content, cloned.content);
        assert_eq!(original.author, cloned.author);
        assert_eq!(original.is_valid, cloned.is_valid);
        assert_eq!(original.visible, cloned.visible);
    }

    #[test]
    fn test_message_debug_output() {
        let msg = Message::new("Test content", "Test author");
        let debug_output = format!("{:?}", msg);

        assert!(debug_output.contains("Test content"));
        assert!(debug_output.contains("Test author"));
    }
}
