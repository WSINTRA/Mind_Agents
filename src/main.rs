// src/main.rs
mod agents;
mod conversation;
mod storage;
mod utils;

use agents::ollama_agent::OllamaAgent;
use agents::team_agent::TeamAgent;
use conversation::room::RoomContext;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Mind Agents System Initializing...");

    let mut room = RoomContext::new();
    let ollama = OllamaAgent::new("qwen3:32b");

    let background_agents = vec![
        TeamAgent::new(
            "Thinking",
            "Representing left brain dominance, logic and analytical thought",
        ),
        TeamAgent::new("Feeling", "Representing feeling, emotion, insight"),
        TeamAgent::new("Intuition", "Representing the unknown, the abstract"),
        TeamAgent::new(
            "Sensation",
            "Representing sensory and value-based processing",
        ),
    ];

    let ego_agent = TeamAgent::new("Executive", "Representing the Ego, conscious integration");

    println!("Enter your prompts (/exit to end):");
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "/exit" {
            match room.save_current_conversation() {
                Ok(path) => println!("Conversation saved to: {}", path.display()),
                Err(e) => eprintln!("Failed to save conversation: {}", e),
            }
            break;
        }

        // Get current conversation context
        let conversation_history = room.get_conversation_summary();

        // Collect background agent responses
        let mut agent_responses = Vec::new();
        for agent in &background_agents {
            // Include conversation history in prompt
            let prompt = format!(
                "Previous conversation:\n{}\n\nCurrent prompt: {}\n\nRespond as {}, {}",
                conversation_history, input, agent.name, agent.agent_prompt
            );

            match ollama.generate_response(&prompt, &mut room).await {
                Ok(response) => {
                    room.update_room_conversation(response.clone(), agent.name.clone());
                    agent_responses.push(format!("{}: {}", agent.name, response));
                }
                Err(e) => {
                    eprintln!("Error from {}: {}", agent.name, e);
                }
            }
        }

        // Create integrated prompt for Executive with full context
        let integrated_prompt = format!(
            "Previous conversation:\n{}\n\nCurrent perspectives on '{}' are:\n{}\n\n\
            As the Executive function, integrate these perspectives into a balanced response \
            that maintains continuity with the previous conversation.",
            conversation_history,
            input,
            agent_responses.join("\n")
        );

        // Get Executive's integrated response
        match ollama
            .generate_response(&integrated_prompt, &mut room)
            .await
        {
            Ok(response) => {
                room.update_room_conversation(response, ego_agent.name.clone());
            }
            Err(e) => {
                eprintln!("Error from Executive: {}", e);
            }
        }

        println!("\nCurrent conversation:");
        println!("{}", room.get_conversation_summary());
        println!("\nToken count: {}", room.get_token_count());
    }

    println!("System shutting down.");
    Ok(())
}
