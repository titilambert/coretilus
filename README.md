# Coretilus

**Coretilus** is a parody of GNU Coreutils — a playful collection of typo-triggered command-line tools written in Rust like https://github.com/uutils/coreutils

Whenever you mistype something like `git` → `gti` or `grep` → `grpe`, Coretilus answers instead of complaining.
Each binary is intentionally useless, funny, or just delightfully absurd.

---

### 🔁 What does “tilus” mean?

> **t.i.l.u.s. = tilus Is Ludicrously Useless Stuff**

A recursive acronym (because of course it is).
It reflects the project’s spirit — *ridiculously, gloriously pointless utilities.*

### 🧩 Included commands

| Command  | Inspired by  | Status       | Description |
|----------|--------------|--------------|-------------|
| `gti`    | `git`        | ✅ - DONE    | “Start your engine!” before committing. |
| `sl`     | `ls`         | ✅ - DONE    | The legendary Steam Locomotive. |
| `mr`     | `rm`         | 🕓 - PLANNED | Land the rocket without crashing it. |
| `pc`     | `cp`         | 🕓 - PLANNED | Copy file from Floppy to IDE using a 486 CPU. |
| `grpe`   | `grep`       | 🕓 - PLANNED | Searches nothing and finds everything. |
| ...adn   | more         | 🕓 - PLANNED | Coming soon, one typo at a time. |
| ...yuor  | own ideas    | 🕓 - PLANNED | Open an issue or a PR. |



---

### 🦀 Why Rust?

Because rewriting useless things in Rust is the most useful way to learn it.
Each mini-utility is a chance to explore a different Rust concept — CLI parsing, ANSI art, async, etc.

### TODOs

* [ ] Add links/credits to original ascii authors
* [ ] Add/Generate man pages
* [ ] Create topydex command
* [ ] Complete tests and increate test coverage
* [ ] Add cargo install support
* [ ] Add brew support

---


### ⚙️ Build

```bash
# sudo apt-get install -y mingw-w64

RUSTFLAGS="-C target-feature=+crt-static" cargo build --release
```

### 🧪 Run tests

```bash
cargo llvm-cov --doctests --open
```

### 📘 Generate docs

```bash
cargo doc --no-deps --open
```
