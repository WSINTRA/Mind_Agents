// src/conversation/runner.rs
use crate::agents::{ollama_agent::OllamaAgent, team_agent::TeamAgent};
use crate::conversation::room::RoomContext;

pub struct ConversationRunner {
    room: RoomContext,
    agents: Vec<TeamAgent>,
    ollama: Option<OllamaAgent>, // Optional to allow gradual integration
}

impl ConversationRunner {
    pub fn new(agents: Vec<TeamAgent>) -> Self {
        Self {
            room: RoomContext::new(),
            agents,
            ollama: None,
        }
    }

    pub fn with_ollama(mut self, model: &str) -> Self {
        self.ollama = Some(OllamaAgent::new(model));
        self
    }

    pub async fn process_single_turn(&mut self, agent_index: usize) -> Result<(), String> {
        let agent = &self.agents[agent_index];

        // First, log the agent activation
        self.room.update_room_conversation(
            format!("{} agent activated", agent.name),
            "System".to_string(),
        );

        // If Ollama is integrated, use it
        if let Some(ollama) = &self.ollama {
            match ollama
                .generate_response(&agent.agent_prompt, &mut self.room)
                .await
            {
                Ok(response) => {
                    self.room
                        .update_room_conversation(response, agent.name.clone());
                    Ok(())
                }
                Err(e) => Err(format!("Ollama generation error: {}", e)),
            }
        } else {
            // Fallback behavior without Ollama
            self.room.update_room_conversation(
                format!("Agent {} would process here", agent.name),
                agent.name.clone(),
            );
            Ok(())
        }
    }

    pub fn get_conversation_summary(&self) -> String {
        self.room.get_conversation_summary()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_conversation_runner_creation() {
        let agents = vec![TeamAgent::new("Test", "Test prompt")];
        let runner = ConversationRunner::new(agents);
        assert!(runner.ollama.is_none());
    }

    #[tokio::test]
    async fn test_conversation_runner_with_ollama() {
        let agents = vec![TeamAgent::new("Test", "Test prompt")];
        let runner = ConversationRunner::new(agents).with_ollama("test_model");
        assert!(runner.ollama.is_some());
    }

    #[tokio::test]
    async fn test_process_single_turn_without_ollama() {
        let agents = vec![TeamAgent::new("Test", "Test prompt")];
        let mut runner = ConversationRunner::new(agents);
        let result = runner.process_single_turn(0).await;
        assert!(result.is_ok());
    }
}
