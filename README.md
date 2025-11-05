# Coretilus

**Coretilus** is a parody of GNU Coreutils — a playful collection of typo-triggered command-line tools written in Rust like https://github.com/uutils/coreutils

Whenever you mistype something like `git` → `gti` or `grep` → `grpe`, Coretilus answers instead of complaining.
Each binary is intentionally useless, funny, or just delightfully absurd.

---

## 🔁 What does “tilus” mean?

> **t.i.l.u.s. = Tilus Is Ludicrously Useless Stuff**

* A recursive acronym (because of course it is).
* It reflects the project’s spirit — *ridiculously, gloriously pointless utilities.*
* It's also `utils` with a typo.

## 🧩 Included commands

| Command  | Inspired by  | Status       | Description |
|----------|--------------|--------------|-------------|
| `sl`     | `ls`         | ✅ - DONE    | The legendary Steam Locomotive.<br>Rust port of https://github.com/mtoyoda/sl |
| `gti`    | `git`        | ✅ - DONE    | “Start your engine!” before committing.<br>Rust port of https://github.com/rwos/gti |
| `mr`     | `rm`         | 🕓 - PLANNED | Land the rocket without crashing it. |
| `pc`     | `cp`         | 🕓 - PLANNED | Copy file from Floppy to IDE using a 486 CPU. |
| `grpe`   | `grep`       | 🕓 - PLANNED | Searches nothing and finds everything. |
| ...adn   | more         | 🕓 - PLANNED | Coming soon, one typo at a time. |
| ...yuor  | own ideas    | 🕓 - PLANNED | Open an issue or a PR. |

---

## ☑️ TODOs

* [ ] Add links/credits to original ascii authors
* [ ] Add/Generate man pages
* [ ] Create topydex command
* [ ] Complete tests and increate test coverage
* [ ] Add brew support

---

## 💻 Install and usage

There are multiple way to install coretilus

### GitHub release artifacts

Go to https://github.com/titilambert/coretilus/releases and download the archive depending on your environment, then extract it like
```
tar xvf coretilus-v0.0.2-linux_x86_64.tar.gz
sl
gti
```
Then move the binaries to folder included in you $PATH, ie /usr/local/bin
```
sudo cp -v sl gti /usr/local/bin/
```

### Using deb package
Go to https://github.com/titilambert/coretilus/releases and download the package depending on your environment, then install it:
```
sudo dpkg -i coretilus_0.0.2-1_amd64.deb
```

### Using rpm package
Go to https://github.com/titilambert/coretilus/releases and download the package depending on your environment, then install it:
```
sudo dnf install coretilus_0.0.2-1_amd64.deb
```



### Using cargo
```
cargo install coretilus
```
Ensure `$HOME/.cargo/bin` is in your `$PATH`, then you can ~~use the binaries~~ do some typos
```
gti
sl
...
```

### MacOs with brew

```
brew tap titilambert/tap
brew install coretilus
```

## ⚙️ Build

```bash
RUSTFLAGS="-C target-feature=+crt-static" cargo build
```

## 🧪 Run tests

```bash
cargo llvm-cov --doctests --open
```

## 📘 Generate docs

```bash
cargo doc --no-deps --open
```
