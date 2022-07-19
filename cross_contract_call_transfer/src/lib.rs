use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    ext_contract, near_bindgen, AccountId, Gas,
};

#[ext_contract(ext_ft)]
trait CalledContract {
    fn ft_transfer(&mut self, receiver_id: String, amount: String, memo: Option<String>);
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
        let ft_contract: AccountId = "my_ft.testnet".parse().unwrap();
        ext_ft::ext(ft_contract)
            .with_attached_deposit(1)
            .ft_transfer("receiver.testnet".to_string(), "2000000".to_string(), None);
    }
}
