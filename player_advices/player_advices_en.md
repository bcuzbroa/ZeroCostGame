# 🎮 Player Guide – ZeroCostGame

Welcome to **ZeroCostGame**!  
This document explains **what you need to install**, **how to prepare the oracle**, and **how to verify your challenges**.

---

## 🛠️ Prerequisites

### Install Rust (rustc)

The game requires the Rust compiler.

Recommended installation via **rustup**:

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
```

Then verify that Rust is correctly installed:


```bash
rustc --version
cargo --version
```

---

## 🔐 Prepare the Oracle

The oracle is the program that validates your solutions and reveals the flags.

Make the oracle executable

From the project root:



```bash
chmod +x oracle

```

To verify it is executable:


```bash
ls -l oracle
```

---

## 🧠 Solving a Challenge

1. Go to the challenges folder.

2. Open the file corresponding to the challenge (e.g., challenge0.rs).

3. Implement the requested function while strictly following the instructions.

## 🧪 Test your code locally

You can run your own tests directly in the main function of the challenge file:
Rust

```rust
fn main() {
    println!("{}", hello());
}
```

## ⚠️ The content of the main function has no impact on validation by the oracle. It is only there to help you test your code. In fact, it is "ignored" when the oracle verifies your challenge.
Verify a challenge with the oracle

Once your code is ready, you can use the oracle to verify your solution:


```bash
./oracle -- -p /path/to/challenge
```

Example:

```bash
 ./oracle -- -p /chemin/vers/challenges/src/challenge0.rs
```

The expected result is as follows:

```bash
✨ Sanity Check ✨ verified: FLAG{XXXX-XXXX-XXXX-XXX}
```

--- 

🚀 Good luck! Read the instructions carefully, and test your code in main. Challenges become increasingly difficult and are worth more and more points.