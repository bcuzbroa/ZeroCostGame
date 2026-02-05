# 🦀 ZeroCostGame: Rust Learning Challenge

Welcome to **ZeroCostGame**! This is a Capture The Flag (CTF) style game designed to help you learn Rust by solving coding puzzles and winning flags.

![ZeroCostGame](assets/logo.png)


---

## 🏗️ Project Architecture

The project is structured as a Rust **Workspace** to keep the player's code and the judge's secrets separate:

* **`challenges/`**: Your playground. This is where you write your solutions.
* **`oracle/`**: The "Black Box". This crate contains the encrypted flags, the challenges code wrapper and the validation logic.

---

## 🚀 How to Play

### 1. Choose your Challenge
Navigate to the `challenges/src/` directory. Each challenge has its own file or module. 
*Example: Open `challenges/src/challenge0.rs`.*

### 2. Solve the Puzzle
Implement the requested logic. It could be a simple return value or a complex algorithm.

```rust
/* =========================================================
CHALLENGE 0: SANITY CHECK
=========================================================
 
1. Implement the "hello" function
@args:    None
@returns: String (owned)

Task: 
Return the exact string "Hello world !".

Note: 
In Rust, there is a difference between a string literal ("...") 
and an owned String. Make sure you return an owned String.
*/

// TODO: Write the hello function here

fn hello() -> String{
    String::from("Hello world !")
}

//Main: Test your challenge here !
fn main() {
    println!("{}", hello())
}

```


# ✅ Verifying a challenge challenge
The oracle is designed to wrapped and verify the player code
`./oracle -- -p <challenge_path>`

Exemple: Verifying the `challenge0.rs` :

```bash
`./oracle -p /your_path/challenge0.rs`
✨ Sanity Check ✨ verified: FLAG{XXXX-XXXX-XXXX-XXX}
```

If this is a success, a flag is prompted, else you are kindly told to keep thinking.

# ⚙️ How to play ?

Generate the script
```bash
sh game_setup.sh
```

An archive is created including all the needed info in `player_advices`


