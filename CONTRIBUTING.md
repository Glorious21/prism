# Contributing to PRISM ✦

Thank you for your interest in contributing to PRISM! We welcome all forms of contribution — bug reports, feature suggestions, documentation improvements, and code.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Coding Conventions](#coding-conventions)
- [Commit Message Format](#commit-message-format)
- [Pull Request Process](#pull-request-process)
- [Issue Labels](#issue-labels)
- [Getting Help](#getting-help)

---

## Code of Conduct

By participating, you agree to follow our [Code of Conduct](CODE_OF_CONDUCT.md). Please treat all contributors with respect.

---

## How to Contribute

### 🐛 Reporting Bugs

1. **Check existing issues** first — [search here](https://github.com/Glorious21/prism/issues).
2. Open a [bug report](https://github.com/Glorious21/prism/issues/new?template=bug_report.md) and fill in all fields.
3. Include: steps to reproduce, expected vs actual behavior, browser/OS info, and screenshots if relevant.

### 💡 Suggesting Features

1. Open a [feature request](https://github.com/Glorious21/prism/issues/new?template=feature_request.md).
2. Describe the problem it solves and why it fits the PRISM vision.

### 🛠 Contributing Code

1. Look for issues labeled [`good first issue`](https://github.com/Glorious21/prism/labels/good%20first%20issue) or [`help wanted`](https://github.com/Glorious21/prism/labels/help%20wanted).
2. Comment on the issue to let us know you are working on it.
3. Follow the setup and PR process below.

---

## Development Setup

### Frontend

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/prism.git
cd prism

# Run a local server
python -m http.server 8085
# Open http://localhost:8085
```

No build step required — the frontend is vanilla HTML/CSS/JS.

### Smart Contracts (Rust / Soroban)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32v1-none

# Install Stellar CLI
cargo install stellar-cli --features opt

# Build the contract
cd contracts/prism
stellar contract build

# Run tests
cargo test
```

---

## Coding Conventions

### JavaScript

- Use `const` and `let` — never `var`
- Prefer `async/await` over raw Promise chains
- Keep functions small and single-purpose
- Add JSDoc comments to all exported functions

```js
/**
 * Connects to Freighter wallet and returns the public key.
 * @returns {Promise<string>} The user's public key
 */
async function connectFreighter() { ... }
```

### CSS

- Follow the existing CSS custom property system (`--prism-*` tokens)
- Use BEM-like class names: `.component__element--modifier`
- Do not add inline styles — use classes or CSS variables
- Mobile-first: base styles for mobile, `@media (min-width: ...)` for larger screens

### Rust (Soroban Contracts)

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Every public function must have doc comments (`///`)
- All error cases must be covered by the `Error` enum
- Run `cargo fmt` and `cargo clippy -- -D warnings` before committing

---

## Commit Message Format

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <short summary>

[optional body]

[optional footer(s)]
```

### Types

| Type | Use when |
|---|---|
| `feat` | Adding a new feature |
| `fix` | Fixing a bug |
| `docs` | Documentation only changes |
| `style` | Formatting, missing semicolons (no logic change) |
| `refactor` | Code change that is not a fix or feature |
| `test` | Adding or fixing tests |
| `chore` | Build process or tooling changes |
| `perf` | Performance improvements |

### Examples

```bash
feat(marketplace): add filtering by asset category
fix(wallet): resolve Freighter connection timeout on Safari
docs(contributing): add soroban contract build steps
style(css): apply Stellar Yellow to CTA hover state
chore(ci): add GitHub Actions workflow for contract build
```

---

## Pull Request Process

1. **Fork** the repo and create your branch from `main`:
   ```bash
   git checkout -b feat/your-feature-name
   ```

2. **Make your changes** following the coding conventions above.

3. **Test your changes** — ensure nothing is broken:
   - Frontend: verify in at least Chrome and Firefox
   - Contracts: run `cargo test`

4. **Commit** using the conventional commit format.

5. **Push** to your fork and **open a Pull Request** targeting `main`.

6. Fill in the PR template completely:
   - What does this PR do?
   - How was it tested?
   - Any screenshots (for UI changes)?
   - Related issue: `Closes #123`

7. A maintainer will review your PR within 3–5 business days. Please be responsive to feedback.

### PR Rules

- PRs must pass all CI checks before merging
- Keep PRs focused — one feature/fix per PR
- Do not force-push to a PR branch once review has started
- Squash commits before merging (maintainer will handle this)

---

## Issue Labels

| Label | Meaning |
|---|---|
| `good first issue` | Great for new contributors |
| `help wanted` | Extra attention needed |
| `bug` | Something isn't working |
| `enhancement` | New feature or improvement |
| `documentation` | Docs improvements |
| `contracts` | Related to Soroban smart contracts |
| `frontend` | Related to HTML/CSS/JS |
| `wontfix` | Will not be addressed |
| `duplicate` | Already reported |

---

## Getting Help

- **GitHub Discussions:** Ask questions and share ideas in [Discussions](https://github.com/Glorious21/prism/discussions)
- **GitHub Issues:** For specific bugs or feature requests
- **Stellar Discord:** [stellar.org/discord](https://stellar.org/discord) — `#soroban-dev` channel

---

Thank you for helping make PRISM better! ✦
