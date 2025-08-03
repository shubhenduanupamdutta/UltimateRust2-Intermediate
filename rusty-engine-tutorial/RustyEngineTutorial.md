# Rusty Engine Tutorial

---

**Rusty Engine** is an open-source game engine created by wrapping a simplification layer around the [Bevy Engine](https://bevy.org/). It is designed to be beginner-friendly, making it easier for new developers to get started with game development in Rust. _Bevy_ is a powerful game engine (real one) that is more complex and has a steeper learning curve, while _Rusty Engine_ aims to provide a more accessible entry point.

**Rusty Engine** purpose in life is to provide a simple Rust interface so you can play around with Rust without the game engine getting in the way.

---

## [Rusty Engine Tutorial Page](https://github.com/CleanCut/rusty_engine)

In the tutorial page, you can find out how to use the engine. We will cover the whole thing in tutorial videos also, but the page will always be up to date, while the videos may not be.

---

## Configuration

---

**1. Add the `rusty_engine` dependency to your `Cargo.toml` file:**

```toml
[dependencies]
rusty_engine = "6.0.0"
```

**2. Download the asset pack:** This will contain all the necessary assets (static) for the tutorial. You can find it in the [Rusty Engine Repo](https://github.com/CleanCut/rusty_engine/archive/refs/heads/main.zip)

**3. Unzip and move the assets:** Unzip the folder, this will give you the whole of the `rusty_engine` repo, then you can move the `assets` folder to your project root. The structure should look like this:

```bash
your_project/
├── Cargo.toml
├── src/
|    ├── main.rs
├── assets/
```

### What is in the assets folder?

- We have **audio** including music (looping) and sound effects (once).
- We have **fonts**, a monospace and a regular font.
- We have **sprites** (images) for racing theme sprites and rolling ball theme sprites.

---

## Engine Initialization

---
