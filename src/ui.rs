use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

use console::style;
use tabled::{Table, Tabled};

use crate::blockchain::Block;

pub fn mining_spinner() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::with_template("{spinner} {msg}").unwrap());
    pb.set_message("Mining block...");
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

#[derive(Tabled)]
struct BlockRow {
    field: String,
    value: String,
}

pub fn print_block(block: &Block, hash: &str) {
    let rows = vec![
        BlockRow {
            field: "Block Hash".into(),
            value: hash.into(),
        },
        BlockRow {
            field: "Timestamp".into(),
            value: block.header.timestamp.to_string(),
        },
        BlockRow {
            field: "Nonce".into(),
            value: block.header.nonce.to_string(),
        },
        BlockRow {
            field: "Prev Hash".into(),
            value: block.header.pre_hash.clone(),
        },
        BlockRow {
            field: "Merkle Root".into(),
            value: block.header.merkle.clone(),
        },
        BlockRow {
            field: "Difficulty".into(),
            value: block.header.difficulty.to_string(),
        },
        BlockRow {
            field: "Tx Count".into(),
            value: block.count.to_string(),
        },
    ];

    println!(
        "\n{}\n{}\n",
        style("⛏️  BLOCK MINED").bold().green(),
        Table::new(rows)
    );

    println!("\n{}", style("📦 Transactions").bold().cyan());

    for (i, tx) in block.transactions.iter().enumerate() {
        println!(
            "{} {} → {} | amount: {}",
            style(format!("#{i}")).yellow(),
            tx.sender,
            tx.reciever,
            tx.amount
        );
    }
    
    println!("")
}
