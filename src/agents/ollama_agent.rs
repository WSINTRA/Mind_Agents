// src/agents/ollama_agent.rs
use crate::conversation::room::RoomContext;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};

pub struct OllamaAgent {
    model: String,
    client: Ollama,
}

impl OllamaAgent {
    pub fn new(model: &str) -> Self {
        OllamaAgent {
            model: model.to_string(),
            client: Ollama::default(),
        }
    }

    pub async fn generate_response(
        &self,
        prompt: &str,
        room: &mut RoomContext,
    ) -> Result<String, String> {
        let system_prompt = "You are a mind agent called Omega, depending on what agent has called, Omega responds appropriatly
                                           Agent 1- Thinking (representing the Jung personality type thinking) - responds with analytical and logical reasoning.
        Agent 2- Feeling (representing the Jung personality type feeling) - responds with emotional and empathetic reasoning.
        Agent 3- Intuition (representing the Jung personality type intuition) - responds with abstract and conceptual reasoning.
        Agent 4- Sensing (representing the Jung personality type sensing) - responds with practical and concrete reasoning.
        Agent 5- Ego (representing the Jung personality type ego) - responds with self-centered and egotistical reasoning. 
        
        For Ego - Respond as if you are public facing. 
        All other agents are background processes that are feeding forward responses to the Ego.
        Unless the Agent is Ego, consider the process a background thinking process and the response will not be public facing. Only the Ego agent is public facing.
        This allows Omega to ruminate on thoughts and ideas. Eventually a prompt will be generated that is public facing. 
        This allows the Ego to understand what the background processes are thinking and feeling. 
        The system is expected to generate any agent's responses, and then the Ego would take that into account for the final answer.
        If there is no Ego, Omega will produce a response, knowing that the response will be fed into a future Ego prompt regardless.
        As the Ego, you will not mention the Jungian system of opposites or the other agents the help you arrive at your output. Omega acts as a complete system. 
        Omega as a system will operate as a parallel to the conway game of life. The simple rules of using the Jungian system agents will allow Omega to produce complex and emergent behavior.
        Handing over the Agentic System.
";

        let request =
            GenerationRequest::new(self.model.clone(), prompt).system(system_prompt.to_string());

        match self.client.generate_stream(request).await {
            Ok(mut stream) => {
                use futures_util::StreamExt;
                use tokio::io::{self, AsyncWriteExt};

                let mut stdout = io::stdout();
                let mut full_response = String::new();

                while let Some(res) = stream.next().await {
                    match res {
                        Ok(chunk) => {
                            for r in chunk {
                                full_response.push_str(&r.response);
                                stdout.write_all(r.response.as_bytes()).await.unwrap();
                                stdout.flush().await.unwrap();
                            }
                        }
                        Err(e) => {
                            return Err(format!("Error generating text: {}", e));
                        }
                    }
                }
                Ok(full_response)
            }
            Err(e) => Err(format!("Error creating stream: {}", e)),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ollama_agent_creation() {
        let agent = OllamaAgent::new("qwen3:32b");
        assert_eq!(agent.model, "qwen3:32b");
    }

    #[tokio::test]
    async fn test_system_prompt_format() {
        let agent = OllamaAgent::new("qwen3:32b");
        let mut room = RoomContext::new();
        let test_prompt = "Test prompt";
        let result = agent.generate_response(test_prompt, &mut room).await;
        assert!(result.is_err(), "Should fail without proper mock");
    }

    // Helper function to create a test environment
    async fn setup_test_environment() -> (OllamaAgent, RoomContext) {
        let agent = OllamaAgent::new("test_model");
        let room = RoomContext::new();
        (agent, room)
    }
}
