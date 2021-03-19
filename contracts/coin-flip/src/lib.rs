use borsh::{ BorshDeserialize, BorshSerialize };
use near_sdk::{
    env, near_bindgen, AccountId, Balance, PublicKey, Promise,
    collections::{ UnorderedMap },
    json_types::{ U64, U128, Base58PublicKey },
};
use serde::Serialize;
use serde::Deserialize;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const ONE_NEAR:u128 = 1_000_000_000_000_000_000_000_000;
const PROB:u8 = 128;

#[near_bindgen]

#[derive(BorshDeserialize, BorshSerialize)]
pub struct SlotMachine {
    pub owner_id: AccountId,
    pub credits: UnorderedMap<AccountId, Balance>,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct HumanReadableGameResult {
    pub user: AccountId,
    pub user_guess: u8,
    pub game_value: u8,
    pub reward_amount: U128,
    pub height: U64,
    pub ts: U64,
}

impl Default for SlotMachine {
    fn default() -> Self {
        panic!("Should be initialized before usage")
    }
}

#[near_bindgen]
impl SlotMachine {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Invalid owner account");
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id,
            credits: UnorderedMap::new(b"credits".to_vec()),
        }
    }

    #[payable]
    pub fn deposit(&mut self) {
        let account_id = env::signer_account_id();
        let deposit = env::attached_deposit();
        let mut credits = self.credits.get(&account_id).unwrap_or(0);
        credits = credits + deposit;
        self.credits.insert(&account_id, &credits);
    }

    #[payable]
    pub fn play(&mut self, choice: u8) -> HumanReadableGameResult {
        let account_id = env::signer_account_id();
        let mut credits = self.credits.get(&account_id).unwrap_or(0);
        assert!(credits > 0, "no credits to play");
        credits = credits - ONE_NEAR;

        let mut gen_choice: u8 = 0;
        let rand: u8 = *env::random_seed().get(0).unwrap();
        if rand < PROB {
            gen_choice = 1;
        }
        let mut result = HumanReadableGameResult {
            user: account_id.clone(),
            user_guess: choice,
            game_value: gen_choice,
            reward_amount: 0.into(),  // if win then update
            height: env::block_index().into(),
            ts: env::block_timestamp().into(),
        };
        if gen_choice == choice {
            credits = credits + 2 * ONE_NEAR;
            result.reward_amount = credits.into();
        }
        let credits_copy = credits.clone();
        credits = 0;
        self.credits.insert(&account_id, &credits);
        Promise::new(account_id).transfer(credits_copy);
        result
    }

    pub fn get_credits(&self, account_id: AccountId) -> U128 {
        self.credits.get(&account_id).unwrap_or(0).into()
    }
}

