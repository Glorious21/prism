<div align="center">

# PRISM ✦

**Create a Prism. Invest in any Prism.**

*A decentralized index basket & financial education protocol built on Stellar Soroban.*

[![License: MIT](https://img.shields.io/badge/License-MIT-FDDA24.svg?style=flat-square&labelColor=0F0F0F)](LICENSE)
[![Stellar](https://img.shields.io/badge/Stellar-Soroban%20Protocol%2027-0F0F0F?style=flat-square&logo=stellar&logoColor=FDDA24)](https://stellar.org)
[![Network](https://img.shields.io/badge/Network-Testnet-blue?style=flat-square)](https://soroban-testnet.stellar.org)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](CONTRIBUTING.md)

[Demo](#-getting-started) · [Report Bug](https://github.com/Glorious21/prism/issues/new?template=bug_report.md) · [Request Feature](https://github.com/Glorious21/prism/issues/new?template=feature_request.md) · [Contribute](CONTRIBUTING.md)

</div>

---

## 📖 Table of Contents

- [About](#-about)
- [Core Features](#-core-features)
- [Architecture](#-architecture)
- [Getting Started](#-getting-started)
- [Smart Contract](#-smart-contract)
- [Contributing](#-contributing)
- [Roadmap](#-roadmap)
- [License](#-license)
- [Acknowledgements](#-acknowledgements)

---

## ✦ About

A physical prism refracts a single ray of white light into a brilliant spectrum of color.

**PRISM Protocol** takes **one single USDC deposit** and automatically refracts it across a full spectrum of tokenized stocks, T-Bills, and commodities on Stellar — in a single atomic Soroban transaction.

- **Minimum investment:** $1.00 USDC
- **Settlement time:** ~3.5 seconds
- **Network fees:** Sub-cent (Stellar)
- **Assets supported:** US Equities (TSLA, AAPL), Treasury yields (USTB, BENJI), Commodities (PAXG), XLM

> PRISM is currently deployed on **Stellar Testnet**. Mainnet launch is on the roadmap.

---

## ✦ Core Features

| Feature | Description |
|---|---|
| 🔬 **Atomic Refraction** | 1-click multi-asset allocation in a single Soroban transaction |
| 🏪 **Index Marketplace** | Browse, create, and invest in community-curated Prism baskets |
| 🎓 **Learn-to-Earn** | Interactive finance lessons with USDC grants upon completion |
| 🧪 **Paper Sandbox** | Risk-free simulator with $10,000 virtual Testnet USDC |
| 🔑 **Multi-Wallet** | Freighter, Albedo, passkey (WebAuthn), and dev keypair support |
| 💸 **Curate-to-Earn** | Basket creators earn 0.10%–0.50% royalty on investments |

---

## ✦ Architecture

```
[ User USDC ] ──► [ PRISM Soroban Engine ] ──► [ Multi-Asset Portfolio ]
                          │
                  ┌───────┴────────┐
                  │  Soroban RPC   │
                  │ (Protocol 27)  │
                  └───────┬────────┘
                          │
          ┌───────────────┼───────────────┐
          ▼               ▼               ▼
      [TSLA SAC]      [PAXG SAC]      [USTB SAC]
```

| Component | Technology | Endpoint |
|---|---|---|
| Smart Contract | Rust (wasm32v1-none) / Soroban Protocol 27 | `contracts/prism/src/lib.rs` |
| Soroban RPC | JSON-RPC 2.0 | `https://soroban-testnet.stellar.org` |
| Stellar Horizon | REST API | `https://horizon-testnet.stellar.org` |
| Friendbot Faucet | Testnet funding | `https://friendbot.stellar.org` |
| Frontend | Vanilla JS + CSS | `index.html`, `style.css`, `script.js` |
| Agent Spec | Machine Payments Protocol (MPP) | `llms.txt` |

---

## ✦ Getting Started

### Prerequisites

- A modern browser (Chrome, Firefox, Brave)
- [Freighter Wallet](https://freighter.app/) browser extension *(optional)*
- Python 3 or any static file server

### Local Development

```bash
# 1. Clone the repository
git clone https://github.com/Glorious21/prism.git
cd prism

# 2. Start a local static server
python -m http.server 8085

# 3. Open in your browser at http://localhost:8085/
```

### Connecting a Wallet

1. Click **"Connect Wallet"** in the top navigation
2. Choose from:
   - **Freighter** — Install the extension, switch to Testnet
   - **Dev Keypair** — Auto-generated, auto-funded via Friendbot (zero setup)
3. Fund your testnet account at [Stellar Laboratory](https://laboratory.stellar.org/#account-creator?network=testnet)

---

## ✦ Smart Contract

The core Soroban contract (`contracts/prism/src/lib.rs`) implements atomic portfolio refraction:

```rust
pub fn refract(
    env: Env,
    from: Address,
    amount: i128,
    basket_id: u32
) -> Result<(), Error> {
    from.require_auth();
    let basket = get_basket(&env, basket_id)?;

    for asset in basket.assets.iter() {
        let share = (amount * asset.bps) / 10_000;
        token::Client::new(&env, &asset.address)
            .transfer(&from, &env.current_contract_address(), &share);
    }
    Ok(())
}
```

### Building the Contract

```bash
# Install the Stellar CLI
cargo install stellar-cli --features opt

# Build
cd contracts/prism
stellar contract build

# Deploy to testnet
stellar contract deploy \
  --wasm target/wasm32v1-none/release/prism.wasm \
  --network testnet \
  --source YOUR_SECRET_KEY
```

---

## ✦ Contributing

We welcome contributions of all kinds — code, documentation, design, and feedback!

Read our **[CONTRIBUTING.md](CONTRIBUTING.md)** to get started.

```bash
# Fork the repo on GitHub, then:
git clone https://github.com/YOUR_USERNAME/prism.git
cd prism
git checkout -b feat/your-feature-name
# Make your changes, then:
git commit -m "feat: describe your change"
git push origin feat/your-feature-name
# Open a Pull Request
```

See [open issues](https://github.com/Glorious21/prism/issues) — look for the `good first issue` label.

---

## ✦ Roadmap

- [x] Prism Simulator (paper trading sandbox)
- [x] Index Marketplace with community baskets
- [x] Multi-wallet support (Freighter, Albedo, passkeys)
- [x] Learn-to-Earn quest engine
- [ ] Soroban contract mainnet deployment
- [ ] Real-time price feeds via Pyth / DIA oracles
- [ ] Governance module (community voting on basket weights)
- [ ] Mobile-responsive progressive web app (PWA)
- [ ] Cross-chain bridge support (CCTP / Axelar)
- [ ] AI portfolio recommendations

---

## ✦ License

Distributed under the **MIT License**. See [`LICENSE`](LICENSE) for more information.

---

## ✦ Acknowledgements

- [Stellar Development Foundation](https://stellar.org) — Soroban smart contract platform
- [Freighter](https://freighter.app/) — Stellar browser wallet
- [Stellar Wallets Kit](https://github.com/Creit-Tech/Stellar-Wallets-Kit) — Multi-wallet support
- [Drips](https://www.drips.network/) — Open source funding

---

<div align="center">
Built with ✦ for the Stellar ecosystem
</div>
