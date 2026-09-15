# PRISM ✦

> **Create a Prism. Invest in any Prism. Learn as you grow.**

```
   _____  _____  _____  _____ __  __ 
  |  __ \|  __ \|_   _|/ ____|  \/  |
  | |__) | |__) | | | | (___ | \  / |
  |  ___/|  _  /  | |  \___ \| |\/| |
  | |    | | \ \ _| |_ ____) | |  | |
  |_|    |_|  \_\_____|_____/|_|  |_|
                                     
  [ $1.00 USDC ] ───► ( PRISM Soroban Engine ) ───► [ Full Spectrum of Stocks & RWAs ]
```

PRISM is a decentralized, community-curated index basket and financial education platform built on the **Stellar blockchain** utilizing **Soroban Smart Contracts (Protocol 27)**. It enables micro-investing from **$1.00** via atomic portfolio refraction across tokenized US equities, Treasury yields, commodities, and digital store-of-value assets with sub-cent network fees.

---

## ✦ The Brand Concept: Why "PRISM"?

A physical prism takes a single ray of white light and refracts it into a brilliant spectrum of color.

**PRISM Protocol** takes **1 single USDC deposit** from a user and automatically **refracts it across a full spectrum of tokenized stocks, T-Bills, and commodities** on Stellar in a single atomic transaction.

* **Headline:** *"Create a Prism. Invest in any Prism."*
* **Design System:** Strictly built according to the official Stellar Brand Guide:
  - **Stellar Yellow** (`#FDDA24`): Primary brand accent, CTAs, and interactive highlights
  - **Stellar Black** (`#0F0F0F`): High-contrast typography and dark surfaces
  - **Pure White** (`#FFFFFF`): Clean editorial canvas
  - **Lora Serif**: Display headlines (`-0.03em` tracking)
  - **Inter Sans**: Crisp UI, navigation, and body copy
  - **JetBrains Mono**: Terminal code blocks and ledger trace logs

---

## ✦ Core Features

### 1. 1-Click Atomic Refraction (Path Payments & SAC)
On traditional brokerages, buying 10 stocks requires high capital minimums and multiple spread markups. PRISM executes multi-asset index allocations in **1 single atomic Soroban transaction**: all legs settle simultaneously in ~3.5 seconds, or the transaction safely reverts.

### 2. Community Index Basket Marketplace
* **Curate Custom Baskets:** Creators set target weight allocations across US equities (TSLA, AAPL), Treasury yields (USTB, BENJI), commodities (PAXG Gold), and native assets (XLM).
* **Curate-to-Earn Yield:** Basket creators earn a 0.10%–0.50% curation royalty whenever community members invest in their published Prism.
* **Filter by Category:** Real-World Assets (RWAs), Inflation Shields, and Crypto & DeFi bluechips.

### 3. Learn-to-Invest Quest Engine (*Duolingo for Wealth*)
* **Interactive 2-Minute Modules:** Learn diversification, dollar-cost averaging, and Stellar Asset Contract (SAC) mechanics.
* **Learn-to-Earn Grants:** Completing interactive quizzes unlocks **$1.00–$5.00 USDC** grants credited directly into the user's sandbox portfolio.

### 4. Paper Trading Sandbox
* **Risk-Free Simulator:** Test and rebalance baskets with **$10,000 virtual Testnet USDC** against live market data without risking real money.
* **Live Holdings Dashboard:** Real-time simulated 30-day returns and portfolio valuation.

### 5. Multi-Wallet & Passkey Support
* **Freighter Wallet:** Native browser extension integration via `@stellar/freighter-api`.
* **Passkeys:** Biometric Face ID / Touch ID smart account authorization via WebAuthn.
* **Albedo Web Signer:** Web-based instant signing protocol.
* **Instant Testnet Dev Keypair:** Zero-friction testnet account creation with live **Friendbot Faucet** funding (+10,000 Testnet XLM).

---

## ✦ Architecture & Endpoints

| Component | Technology | Live Endpoint |
|---|---|---|
| **Soroban Smart Contract** | Rust (`wasm32v1-none`) Protocol 27 | `contracts/prism/src/lib.rs` |
| **Soroban RPC** | JSON-RPC 2.0 | `https://soroban-testnet.stellar.org` |
| **Stellar Horizon** | REST API | `https://horizon-testnet.stellar.org` |
| **Friendbot Faucet** | Testnet Funding | `https://friendbot.stellar.org` |
| **Agent / MCP Spec** | Machine Payments Protocol (MPP) | `/llms.txt` |
| **Frontend** | Vanilla JS + High-Taste CSS | `index.html`, `style.css`, `script.js` |

---

## ✦ Smart Contract Anatomy

The core contract (`contracts/prism/src/lib.rs`) implements atomic refraction using Soroban's SAC interface:

```rust
pub fn refract(
    env: Env, 
    from: Address, 
    amount: i128, 
    basket_id: u32
) -> Result<(), Error> {
    from.require_auth();
    let basket = get_basket(&env, basket_id)?;
    
    // Single-transaction atomic SAC transfer across target assets
    for asset in basket.assets.iter() {
        let share = (amount * asset.bps) / 10_000;
        token::Client::new(&env, &asset.address)
            .transfer(&from, &env.current_contract_address(), &share);
    }
    Ok(())
}
```

---

## ✦ Getting Started Locally

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Glorious21/prism.git
   cd prism
   ```

2. **Run local static server:**
   ```bash
   # Python 3
   python -m http.server 8085
   ```

3. **Open in browser:**
   Navigate to `http://localhost:8085/`

---

## ✦ License

Apache 2.0 & MIT — Built for the Stellar Community.
