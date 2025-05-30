// src/agents/ollama_agent.rs
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

    pub async fn generate_response(&self, prompt: &str) -> Result<String, String> {
        let system_prompt = "You are a mind systemn called Omega, depending on what agent has called, Omega responds appropriatly
                                          All other agents except Output are background processes that are feeding forward responses to the Ego.
        Unless the Agent is Ego, consider the process a background thinking process and the response will not be public facing. Only the Ego agent is public facing.
        This allows Omega to ruminate on thoughts and ideas.
        If there is no Ego, Omega will produce a response, knowing that the response will be fed into a future Ego prompt regardless.
        As the Output, you will not mention the Jungian system of opposites or the other agents the help you arrive at your output.
        You do not need to break out the quaternity into summaries, instead when in Output, form a consistent output that integrates the opinions that Omega decides are most relevant in the situation. Omega acts as a complete system. 
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
        let test_prompt = "Test prompt";
        let result = agent.generate_response(test_prompt).await;
        assert!(result.is_err(), "Should fail without proper mock");
    }
}
