// src/agents/team_agent.rs
#[derive(Debug, Clone)]
pub struct TeamAgent {
    pub name: String,
    pub agent_prompt: String,
}

impl TeamAgent {
    pub fn new(name: &str, agent_prompt: &str) -> Self {
        TeamAgent {
            name: name.to_string(),
            agent_prompt: agent_prompt.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_agent_creation() {
        let agent = TeamAgent::new(
            "Thinking",
            "Representing left brain dominance, logic and analytical thought",
        );

        assert_eq!(agent.name, "Thinking");
        assert_eq!(
            agent.agent_prompt,
            "Representing left brain dominance, logic and analytical thought"
        );
    }

    #[test]
    fn test_team_agent_clone() {
        let agent = TeamAgent::new("Feeling", "Representing feeling, emotion, insight");
        let cloned_agent = agent.clone();

        assert_eq!(agent.name, cloned_agent.name);
        assert_eq!(agent.agent_prompt, cloned_agent.agent_prompt);
    }

    #[test]
    fn test_team_agent_debug_output() {
        let agent = TeamAgent::new("Intuition", "Representing the unknown, the abstract");

        let debug_output = format!("{:?}", agent);
        assert!(debug_output.contains("Intuition"));
        assert!(debug_output.contains("Representing the unknown, the abstract"));
    }
}
