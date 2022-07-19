use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, ext_contract, log, near_bindgen, AccountId, Gas,
};

#[ext_contract(ext_called_contract)]
trait CalledContract {
    fn get_signer_id() -> AccountId;
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {}

impl Default for Contract {
    fn default() -> Self {
        Self {}
    }
}

pub const GAS_FEE: Gas = Gas(5_000_000_000_000);

#[near_bindgen]
impl Contract {
    pub fn cross_contract_call(&mut self) {
        let called_contract_ac: AccountId = "final_called_contract_ac.testnet".parse().unwrap();
        ext_called_contract::ext(called_contract_ac)
            .get_signer_id()
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FEE)
                    .callback(),
            );
    }

    #[private]
    pub fn callback(#[callback_unwrap] id: AccountId) {
        log!("got: {}", id);
    }
}
