<div align="center">

![uutils logo](coretilus.svg)


# coretilus


</div>

**coretilus** is a parody of GNU Coreutils — a playful collection of typo-triggered command-line tools written in Rust like https://github.com/uutils/coreutils

Whenever you mistype something like `git` → `gti` or `grep` → `grpe`, coretilus answers instead of complaining.
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
| `pc`     | `cp`         | ✅ - DONE    | Because data deserves a grand tour of your 486 before reaching the disk |
| `mr`     | `rm`         | ✅ - DONE    | Land the rocket without crashing it. <br> https://ascii.co.uk/art/rockets / https://ascii.co.uk/art/explosion |
| `dog`    | `dig`        | ✅ - DONE    | A dog chasing a domain. <br> https://www.asciiart.eu/animals/dogs |
| `gb`     | `bg`         | ✅ - DONE    | Playing retro video game instead of running background job |
| `ehco`   | `echo`       | ✅ - DONE    | A parrot trying to repeat <br> https://www.asciiart.eu/search?q=parrot |
| `grpe`   | `grep`       | 🕓 - PLANNED | Searches nothing and finds everything. |
| ...adn   | more         | 🕓 - PLANNED | Coming soon, one typo at a time. |
| ...yuor  | own ideas    | 🕓 - PLANNED | Open an issue or a PR. |

---

## ☑️ TODOs

* [ ] Add links/credits to original ascii authors (WIP)
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

## 🛠️ Development tools

Everything below can be set up with only [`rustup`](https://rustup.rs/)
installed (no other package required). These tools are **development-only**
(not project dependencies, nothing to add to `Cargo.toml`) and are used by
the `pre-commit` hooks and CI.

```bash
# 1. Stable toolchain + formatting/linting components
rustup component add rustfmt clippy

# 2. Nightly toolchain (required by cargo-udeps to detect unused dependencies)
rustup toolchain install nightly

# 3. Coverage report
cargo install cargo-llvm-cov --locked
# If cargo-llvm-cov can't find llvm-cov/llvm-profdata, pointing it to your
# LLVM toolchain may be necessary:
export LLVM_COV=$(which llvm-cov)
export LLVM_PROFDATA=$(which llvm-profdata)

# 4. Security audit
cargo install cargo-audit --locked

# 5. Detect unused dependencies (runs against the nightly toolchain from step 2)
cargo install cargo-udeps --locked

# 6. Git hooks runner (Rust-native pre-commit alternative, no Python needed)
cargo install prek --locked
prek install
```

If a tool or the nightly toolchain is missing, the corresponding pre-commit
hook is skipped with a warning instead of failing the commit.

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

## Generate gifs

First, we need to compile then we use https://github.com/charmbracelet/vhs

```bash
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release
docker run --rm -v `pwd`/gifs:/vhs/output -v `pwd`:/src:ro -v `pwd`/target/release:/usr/local/bin ghcr.io/charmbracelet/vhs /src/coretilus.tape
for COMMAND in $(ls -d src/commands/*/ | xargs -n1 basename)
do
docker run --rm -v `pwd`/gifs:/vhs/output -v `pwd`/src:/src:ro -v `pwd`/target/release:/usr/local/bin ghcr.io/charmbracelet/vhs /src/commands/${COMMAND}/${COMMAND}.tape
done
```
### Optimize gifs if needed

```
gifsicle -O3 --colors 256 <heavy_gif>gif -o <light_gif>.gif
```
