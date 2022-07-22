# cross_contract_call_transfer
I would like to have a certain contract act as an intermediary for the transfer of tokens.  
This is I tried:  
1. create 4 accounts and deployed contracts.
- `sender.testnet`: token sender
- `receiver.testnet`: token receiver
- `mediator.testnet`: contracts for which I would like to be the intermediary is deployed
- `my_ft.testnet`: my fungible token contract is deployed
2. the code of mediator contract is following:
    
    ```bash
    #[ext_contract(ext_ft)]
    trait FTContract {
        fn ft_transfer(&mut self, receiver_id: String, amount: String, memo: Option<String>);
    }
    
    #[near_bindgen]
    impl Contract {
        pub fn cross_contract_call(&mut self) {
            let ft_contract: AccountId = "my_ft.testnet".parse().unwrap();
            ext_ft::ext(ft_contract)
                .with_attached_deposit(1)
                .ft_transfer("receiver.testnet".to_string(), "2000000".to_string(), None);
        }
    }
    ```
    
3. execute :
    
    ```bash
    near call mediator.testnet cross_contract_call '' --accountId sender.testnet
    ```
    

But In this case, tokens are transferred from `mediator.testnet` to `receiver.testnet`, not from `sender.testnet` to `receiver.testnet`.  

Is there any good way to transfer money from `sender.testnet` to `receiver.testnet` through `mediator.testnet` ?  
Or do `sender.testnet` have to call ft_transfer() in `my_ft.testnet` directly to transfer tokens to `receiver.testnet`?  

Thanks for reading.

# cross_contract_call + final_called_contract
Can one account call method on behalf of another account on NEAR Protocol?

I have implemented a cross contract call.

Then I have prepared three accounts and two contracts.

1. account: `signer_ac.testnet`
A account calls the cross-contract call method described next.
2. account: `cross_contract_call_ac.testnet`
A account where the contract with the cross-contract call method is deployed.
And the code of this contract is following:
    
    ```rust
    #[ext_contract(ext_called_contract)]
    trait CalledContract {
        fn get_signer_id() -> AccountId;
    }
    
    #[near_bindgen]
    #[derive(BorshDeserialize, BorshSerialize)]
    pub struct Contract {}
    
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
    ```
    
    Log the return value of the method called by this method.
    
3. account: `final_called_contract_ac.testnet`
A account where the contract called by the above method (cross contract call) is deployed.
And the code of this contract is following:
    
    ```rust
    pub struct Contract {}
    
    #[near_bindgen]
    impl Contract {
        pub fn get_signer_id(&mut self) -> AccountId {
            env::predecessor_account_id()
        }
    }
    ```
    
    Return the name of the account that call this method
    

And I executed:

```bash
$ near call cross_contract_call_ac.testnet cross_contract_call '' --accountId signer_ac.testnet

--> Log [cross_contract_call_ac.testnet]: got: cross_contract_call_ac.testnet
```

I understand this result because the get_signer_id method on `final_called_contract_ac.testnet` was called by `cross_contract_call_ac.testnet`

But can't get_signer_id also be called by signer_ac.testnet?
I mean, in this situation, 

First, `signer_ac.testnet` call cross_contract_call on `cross_contract_call_ac.testnet`.

Then, `cross_contract_call_ac.testnet` call get_signer_id on  `final_called_contract_ac.testnet` as `signer_ac.testnet`.

Finally, get_signer_id return `signer_ac.testnet` as value.
