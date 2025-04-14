# 🎮 Pong in Rust

A classic **Pong** game implemented in Rust using the awesome [`bracket-lib`](https://github.com/amethyst/bracket-lib)!  
This version features smooth ball physics, paddle collisions, basic AI, and scorekeeping.

![pong banner](https://user-images.githubusercontent.com/73541678/132239808-3d8e0e3e-4d98-4e17-b1e7-8d8e7e20646f.gif)

## 🚀 Features

- 🏓 Two players: Human (keyboard) vs AI (auto-tracking)
- 🎯 Ball mechanics with wall and paddle collisions
- 💥 Scorekeeping and win detection
- 🕹️ Simple main menu
- 🔁 Game resets after each score


## 🧰 Dependencies

- [`bracket-lib`](https://crates.io/crates/bracket-lib) (Roguelike and retro game engine)

You can add it to your project by including this in your `Cargo.toml`:

```toml
[dependencies]
bracket-lib = "0.8"
```

## 🛠️ How to Run From Source
1. Clone this repo:
```
git clone https://github.com/hmousavin/rust-pong.git
cd rust-pong
```

2. Build and run the game:
```
cargo run
```
