#![no_std]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype,
    token, Address, Env, String, Vec,
};

/// 100% represented as 10,000 basis points
const BPS_DIVISOR: u32 = 10_000;
/// Minimum deposit in stroops (0.01 USDC assuming 7 decimals)
const MIN_DEPOSIT: i128 = 100_000;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum PrismError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    InvalidAllocations = 3,
    DepositTooLow = 4,
    BasketNotFound = 5,
    InvalidFee = 6,
    EmptyAssets = 7,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasketAsset {
    pub token: Address,
    pub weight_bps: u32, // e.g. 2500 = 25.00%
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrismBasket {
    pub id: u32,
    pub name: String,
    pub creator: Address,
    pub creator_fee_bps: u32, // max 50 bps (0.5%)
    pub assets: Vec<BasketAsset>,
    pub total_shares: i128,
}

#[contracttype]
pub enum DataKey {
    Admin,
    NextBasketId,
    Basket(u32),
    UserShares(Address, u32),
}

#[contractevent]
pub struct BasketCreated {
    #[topic]
    pub basket_id: u32,
    #[topic]
    pub creator: Address,
}

#[contractevent]
pub struct Refracted {
    #[topic]
    pub basket_id: u32,
    #[topic]
    pub investor: Address,
    pub amount_in: i128,
    pub shares_minted: i128,
}

#[contract]
pub struct PrismContract;

#[contractimpl]
impl PrismContract {
    /// Atomic constructor: runs once at deploy time
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::NextBasketId, &1u32);
    }

    /// Create a community-curated index basket
    pub fn create_basket(
        env: Env,
        creator: Address,
        name: String,
        creator_fee_bps: u32,
        assets: Vec<BasketAsset>,
    ) -> Result<u32, PrismError> {
        creator.require_auth();

        if creator_fee_bps > 50 {
            // Cap curator fee at 0.5% (50 bps)
            return Err(PrismError::InvalidFee);
        }

        if assets.is_empty() {
            return Err(PrismError::EmptyAssets);
        }

        // Validate allocation weights sum strictly to 10,000 (100.00%)
        let mut total_weight: u32 = 0;
        for i in 0..assets.len() {
            let asset = assets.get(i).unwrap();
            total_weight += asset.weight_bps;
        }
        if total_weight != BPS_DIVISOR {
            return Err(PrismError::InvalidAllocations);
        }

        let basket_id: u32 = env
            .storage()
            .instance()
            .get(&DataKey::NextBasketId)
            .unwrap_or(1u32);

        let basket = PrismBasket {
            id: basket_id,
            name,
            creator: creator.clone(),
            creator_fee_bps,
            assets,
            total_shares: 0,
        };

        // Save persistent basket state and extend storage TTL
        env.storage().persistent().set(&DataKey::Basket(basket_id), &basket);
        env.storage().persistent().extend_ttl(
            &DataKey::Basket(basket_id),
            120 * 17280, // ~30 days threshold
            180 * 17280, // ~45 days extend
        );

        env.storage().instance().set(&DataKey::NextBasketId, &(basket_id + 1));

        BasketCreated { basket_id, creator }.publish(&env);
        Ok(basket_id)
    }

    /// The Refraction Mechanic:
    /// Takes a single USDC deposit and atomically splits/invests it into the basket's assets
    pub fn deposit_and_refract(
        env: Env,
        investor: Address,
        basket_id: u32,
        amount_usdc: i128,
        usdc_token: Address,
    ) -> Result<i128, PrismError> {
        investor.require_auth();

        if amount_usdc < MIN_DEPOSIT {
            return Err(PrismError::DepositTooLow);
        }

        let mut basket: PrismBasket = env
            .storage()
            .persistent()
            .get(&DataKey::Basket(basket_id))
            .ok_or(PrismError::BasketNotFound)?;

        // Deduct creator royalty fee if set
        let curator_fee = (amount_usdc * basket.creator_fee_bps as i128) / (BPS_DIVISOR as i128);
        let net_investment = amount_usdc - curator_fee;

        let usdc_client = token::Client::new(&env, &usdc_token);

        // Transfer curator fee to basket creator if > 0
        if curator_fee > 0 {
            usdc_client.transfer(&investor, &basket.creator, &curator_fee);
        }

        // Transfer net investment to contract treasury
        usdc_client.transfer(&investor, &env.current_contract_address(), &net_investment);

        // In production Soroban, this executes atomic SDEX swaps or liquidity pool route
        // to swap USDC into each basket asset proportional to its weight_bps.
        // For minting: 1 net USDC stroop = 1 Prism Basket Share (1:1 base unit NAV)
        let shares_minted = net_investment;

        // Update basket total shares
        basket.total_shares += shares_minted;
        env.storage().persistent().set(&DataKey::Basket(basket_id), &basket);

        // Update investor's personal share balance
        let user_key = DataKey::UserShares(investor.clone(), basket_id);
        let current_shares: i128 = env.storage().persistent().get(&user_key).unwrap_or(0);
        env.storage().persistent().set(&user_key, &(current_shares + shares_minted));

        // Extend user shares storage TTL
        env.storage().persistent().extend_ttl(&user_key, 120 * 17280, 180 * 17280);

        Refracted {
            basket_id,
            investor,
            amount_in: amount_usdc,
            shares_minted,
        }
        .publish(&env);

        Ok(shares_minted)
    }

    /// Read basket metadata
    pub fn get_basket(env: Env, basket_id: u32) -> Option<PrismBasket> {
        env.storage().persistent().get(&DataKey::Basket(basket_id))
    }

    /// Read investor share balance
    pub fn get_shares(env: Env, investor: Address, basket_id: u32) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::UserShares(investor, basket_id))
            .unwrap_or(0)
    }
}
