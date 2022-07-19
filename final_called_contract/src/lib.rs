use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen, AccountId,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {}

impl Default for Contract {
    fn default() -> Self {
        Self {}
    }
}

#[near_bindgen]
impl Contract {
    pub fn get_signer_id(&mut self) -> AccountId {
        env::predecessor_account_id()
    }
}
