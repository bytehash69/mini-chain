mod blockchain;
mod commands;
mod prompt;
mod ui;

use blockchain::Chain;
use commands::CommandGroup;
use console::style;
use prompt::{prompt, prompt_input_data};

fn main() -> anyhow::Result<()> {
    println!("{}", style("⚡ Mini-chain - A tiny, playful blockchain in your terminal").bold().green());

    let miner_addr: String = prompt_input_data("Enter miner address:");
    let difficulty: u32 = prompt_input_data("Enter initial difficulty:");
    let reward: u32 = prompt_input_data("Enter initial reward:");

    let mut chain = Chain::new(miner_addr, difficulty, reward);

    loop {
        let cmd = prompt()?;

        match cmd {
            CommandGroup::NewTransaction => {
                let sender: String = prompt_input_data("Sender:");
                let receiver: String = prompt_input_data("Receiver:");
                let amount: f32 = prompt_input_data("Amount:");

                chain.new_transaction(sender, receiver, amount);
            }

            CommandGroup::MineBlock => {
                chain.generate_new_block();
            }

            CommandGroup::ChangeDifficulty => {
                let diff: u32 = prompt_input_data("New difficulty:");
                chain.update_difficulty(diff);
            }

            CommandGroup::ChangeReward => {
                let reward: f32 = prompt_input_data("New mining reward:");
                chain.update_reward(reward);
            }

            CommandGroup::Exit => break,
        }
    }

    Ok(())
}
