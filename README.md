# Mini-Chain CLI — A Tiny, Playful Blockchain in Your Terminal.

A lightweight, interactive command-line blockchain simulator written in **Rust**, designed to demonstrate **core blockchain concepts** such as transactions, blocks, proof-of-work mining, merkle roots, and chain state. All with a polished CLI experience.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust)
![CLI](https://img.shields.io/badge/CLI-Blockchain-blue?style=for-the-badge)
![PoW](https://img.shields.io/badge/PoW-Mining-orange?style=for-the-badge)

---

## 🚀 Key Features

- **Interactive CLI** using `inquire`
- **Proof-of-Work Mining** with configurable difficulty
- **Merkle Tree Construction** for transaction integrity
- **Mining Rewards** automatically added per block
- **Pretty Block Output** using tables & boxed layouts
- **Runtime Configuration** (difficulty & reward)
- **Educational & Inspectable** — no hidden magic

---

## 🖥️ Demo

### Mining a Block

```

⛏️  BLOCK MINED
+-------------+------------------------------------------------------------------+
| field       | value                                                            |
+-------------+------------------------------------------------------------------+
| Block Hash  | 000dfd975967784aab632be4b77a4f5762e65b7615ea8da5eff099f9fd92     |
+-------------+------------------------------------------------------------------+
| Timestamp   | 1767717755                                                       |
+-------------+------------------------------------------------------------------+
| Nonce       | 112834                                                           |
+-------------+------------------------------------------------------------------+
| Prev Hash   | 006252df80d77b684ca05dfc5637d6ea41a787ee52ea18986fc1fa711773     |
+-------------+------------------------------------------------------------------+
| Merkle Root | 573c8d775b99dcf83151d427c4bbfa77c81fb069295f8df35b772194e71295fb |
+-------------+------------------------------------------------------------------+
| Difficulty  | 2                                                                |
+-------------+------------------------------------------------------------------+
| Tx Count    | 2                                                                |
+-------------+------------------------------------------------------------------+


📦 Transactions
#0 root → root | amount: 2
#1 toly → raj  | amount: 15

```

---

## **Commands**

| Command               | What it does               | Status |
| --------------------- | -------------------------- | ------ |
| **New transaction**   | Create a new transaction   | Done   |
| **Mine block**        | Mine a new block with PoW  | Done   |
| **Change difficulty** | Update mining difficulty   | Done   |
| **Change reward**     | Update mining reward       | Done   |
| **Exit**              | Exit the CLI               | Done   |

---

## **Quick Start**

### **1. Install**

```bash

# Clone and build
git clone https://github.com/bytehash69/mini-chain.git
cd mini-chain
cargo install --path .

```
---

## **2. Usage**

Launch Mini-Chain :

```bash
mini-chain
```

**Navigate using arrow keys, press **Enter** to select.**

```bash
⚡ Mini-Chain — A Tiny Blockchain in Your Terminal

? Choose a command:
  > New transaction
    Mine block
    Change difficulty
    Change reward
    Exit
```

Mining a block:

```bash

? Choose a command: Mine block
⠋ Mining... nonce 15069

✔ Block mined!

```

Creating a transaction:

```bash

? Choose a command: New transaction
? Sender: toly
? Receiver: raj
? Amount: 10

✔ Transaction added

```

---

## 🏗️ Project Structure

```
src/
├── main.rs          # Application entry point
├── blockchain.rs    # Core blockchain logic
├── prompt.rs        # User input handling (inquire)
├── ui.rs            # CLI visuals (spinner, tables)
└── commands/
    └── mod.rs       # Command definitions
```

---

## 🧠 Learning Goals

This project is built to help understand:

* How blocks link via hashes
* How Proof-of-Work functions internally
* How Merkle roots secure transactions
* How state is managed in a blockchain
* How to build clean Rust CLIs

---

## 🚧 Limitations (By Design)

* Not networked (single-node)
* No persistence (in-memory only)
* Not cryptographically hardened
* Uses floats for amounts (educational)

---

## 🤝 Contributing

Contributions, improvements, and refactors are welcome.

---

## 📄 License

MIT License
