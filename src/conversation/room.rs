// src/conversation/room.rs
use super::message::Message;
use crate::storage::transcript::TranscriptManager;
use crate::utils::token_counter::TokenCounter;
use std::io; // Add this import
use std::path::PathBuf;
#[derive(Debug)]
pub struct RoomContext {
    messages: Vec<Message>,
    token_counter: TokenCounter,
    total_tokens: usize,
}

impl RoomContext {
    pub fn new() -> Self {
        RoomContext {
            messages: Vec::new(),
            token_counter: TokenCounter::new(32768), // Qwen's context length
            total_tokens: 0,
        }
    }
    pub fn save_current_conversation(&self) -> io::Result<PathBuf> {
        let manager = TranscriptManager::new()?;
        manager.save_transcript(&self.get_conversation_summary())
    }

    pub fn update_room_conversation(&mut self, content: String, author: String) {
        // Check if adding this message would exceed token limit
        if self
            .token_counter
            .would_exceed_limit(self.total_tokens, &content)
        {
            // Remove oldest messages until we have space
            while self
                .token_counter
                .would_exceed_limit(self.total_tokens, &content)
                && !self.messages.is_empty()
            {
                // Fix: remove() returns Option<T>, so we need to handle that
                if let Some(removed_msg) = self.messages.first() {
                    let tokens_to_remove = self.token_counter.estimate_tokens(&removed_msg.content);
                    self.total_tokens -= tokens_to_remove;
                    self.messages.remove(0);
                }
            }
        }

        println!("\n{}: {}\n\n", author, content);
        let new_tokens = self.token_counter.estimate_tokens(&content);
        self.total_tokens += new_tokens;

        self.messages.push(Message::new(&content, &author));
    }

    pub fn get_token_count(&self) -> usize {
        self.total_tokens
    }

    // Existing methods remain the same
    pub fn read(&self) -> Vec<&Message> {
        self.messages.iter().filter(|m| m.is_valid).collect()
    }

    pub fn get_conversation_summary(&self) -> String {
        self.read()
            .iter()
            .map(|msg| format!("{}: {}", msg.author, msg.content))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_limit_management() {
        let mut room = RoomContext::new();

        // Add a large message
        let large_message = "a".repeat(4000); // ~1000 tokens
        room.update_room_conversation(large_message, "User".to_string());
        assert_eq!(room.get_token_count(), 1000);

        // Add messages until we approach limit
        for i in 0..30 {
            let msg = "a".repeat(4000);
            room.update_room_conversation(msg, format!("User{}", i));
        }

        // Verify oldest messages were removed
        assert!(room.get_token_count() < 32768);
    }

    // Existing tests remain the same
    #[test]
    fn test_room_creation() {
        let room = RoomContext::new();
        assert!(room.messages.is_empty());
    }

    #[test]
    fn test_add_message() {
        let mut room = RoomContext::new();
        room.update_room_conversation("Hello".to_string(), "User".to_string());
        assert_eq!(room.messages.len(), 1);
        assert_eq!(room.messages[0].content, "Hello");
        assert_eq!(room.messages[0].author, "User");
    }
}
