# 🧭 GTE — GitHub TUI Explorer

GTE is a fast, async terminal dashboard for exploring GitHub repositories, issues, and pull requests — all from your terminal.

---

## Features

- Display repositories with stars, forks, and open issues
- Multiple pages: Repos, Issues, Pull Requests
- Keyboard navigation
- Built in Rust using async and TUI

---

## Configuration

Create a `.env` file in the project root:

```
GITHUB_USERNAME=yourusername
GITHUB_TOKEN=your_personal_access_token
```

---

## Installation

```bash
git clone [https://github.com/yourusername/gte.git](https://github.com/leonardo3130/github-tui-explorer.git)
cd gte
cargo build
cargo run
```

---

## Usage

- `q`: Quit the app
- `s`: Search repos

---

## TODO

- show issues for a repo
- show PRs for a repo
- ...
