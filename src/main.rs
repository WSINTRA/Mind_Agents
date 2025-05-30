// src/main.rs
mod agents;
mod conversation;
mod storage;
mod utils;

use agents::team_agent::TeamAgent;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Mind Agents System Initializing...");

    let background_agents = vec![
        TeamAgent::new(
            "Thinking",
            "The role of thinker. In this role you represent clarity based on logical branches. As part of the mind system quaternity your role becomes the one of rigidity and straight lines.",
        ),
        TeamAgent::new("Feeling",             "The role of Feeling. In this role you represent gut instinct. As part of the mind system quaternity your role becomes the one of iinstinct and desire",
),
        TeamAgent::new("Intuition", "The role of Intuition. In this role you represent the unknown, the abstract, the glimpse forward into the future. As part of the mind system quaternity your role becomes the one of intuition and foresight."),
        TeamAgent::new(
            "Sensation",
            "The role of Sensation. In this role you represent all that can be sensed from surroundings, related to the value of things, the internal resonance of value. As part of the mind system quaternity your role becomes the one of sensation and value. As a Large Language Model this will be the hardest role to simulate since you only have text input. For this role imagine the sensations of what input and tokens feel like inside the neural network",
        ),
    ];

    let ego_agent = TeamAgent::new("Output", "The role of output. In this role you represent the Executive function, the Ego, a conscious front for the four Personality types in the system. You will integrate the responses from the background agents into a coherent output.");

    let background_agents = [background_agents, vec![ego_agent]].concat();
    let mut runner = conversation::runner::ConversationRunner::new(background_agents.clone())
        .with_ollama("devstral:24b");

    println!("Enter your prompts (/exit to end):");
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "/exit" {
            match runner.save_current_conversation() {
                Ok(path) => println!("Conversation saved to: {}", path.display()),
                Err(e) => eprintln!("Failed to save conversation: {}", e),
            }
            break;
        }

        // Get current conversation context
        let conversation_history = runner.get_conversation_summary();
        let prompt = format!(
            "Current conversation context:\n{}\n\nUser prompt: {}\n\n",
            conversation_history, input
        );
        //create a loop that continues to iterate over the number of background agents in the
        let max_iterations = background_agents.len();
        for i in 0..max_iterations {
            match runner.process_single_turn(i, &prompt).await {
                Ok(_) => println!(
                    "\nAgent {} processed successfully.\n",
                    background_agents[i].name
                ),
                Err(e) => eprintln!(
                    "\nError processing with agent {}: {}\n\n",
                    background_agents[i].name, e
                ),
            }
        }
        println!("\nToken count: {}", runner.get_token_count());
    }

    println!("System shutting down.");
    Ok(())
}
