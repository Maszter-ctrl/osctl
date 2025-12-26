# osctl

**Declarative OS Configuration Tool**

`osctl` is a Rust-based command-line tool that lets you manage your macOS system declaratively. Instead of manually installing packages or enabling services, you define your desired system state in a YAML file (`os.yaml`) and let `osctl` apply the changes.

---

## Features

- Install packages declaratively
- Enable or disable system services
- Preview changes with `--dry-run`
- Generate a diff between current and desired state
- Fully automated system setup for new machines or team environments

---

## Installation

You need [Rust](https://www.rust-lang.org/tools/install) installed. Then, clone this repo and build:

Run the program with: cargo run -- [COMMAND]

For example expecting:

packages:
  install:
    - git
    - curl
    - wget
    - node

services:
  enable:
    - ssh
  disable:
    - bluetooth


```bash
git clone https://github.com/yourusername/osctl.git
cd osctl
cargo build --release
