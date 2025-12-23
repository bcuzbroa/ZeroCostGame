# 🦀 ZeroCostGame: Rust Learning Challenge

Welcome to **ZeroCostGame**! This is a Capture The Flag (CTF) style game designed to help you learn Rust by solving coding puzzles.

---

## 🏗️ Project Architecture

The project is structured as a Rust **Workspace** to keep the player's code and the judge's secrets separate:

* **`challenges/`**: Your playground. This is where you write your solutions.
* **`oracle/`**: The "Black Box". This crate contains the encrypted flags and the validation logic. It depends on your code to work.



---

## 🚀 How to Play

### 1. Choose your Challenge
Navigate to the `challenges/src/` directory. Each challenge has its own file or module. 
*Example: Open `challenges/src/challenge0.rs`.*

### 2. Solve the Puzzle
Implement the requested logic. It could be a simple return value or a complex algorithm.

```rust
// challenges/src/challenge0.rs
// hello function must return a string which contains "Hello World !"
pub fn hello() -> ........ {
    // YOUR CODE HERE
}
```


# To verify Challenge 0
`cargo run -p oracle -- 0`
If this is a success, a flag is prompted, else you are kindly told to keep thinking.


## Admin :

Once the ChallengeVerifier trait is implemented for a specifique challenge, one can generate a encrypted flag using:
`cargo run --bin encrypt_flag -- <challenge_id> <flag plaintext>`
