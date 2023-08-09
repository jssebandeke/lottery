use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, env, require, Promise, ONE_NEAR, log};
use near_sdk::store::{UnorderedSet,LookupMap};
use near_sdk::json_types::U128;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Lottery {

    pub lottery_id: u32,
    pub owner: AccountId,
    pub players: UnorderedSet<AccountId>,
    pub winners: LookupMap<u32, AccountId>
    
}

impl Default for Lottery {
    fn default() -> Self {
        Self { 
               owner: env::signer_account_id() ,
               players: UnorderedSet::new(b'i'), 
               lottery_id: Default::default(),
               winners: LookupMap::new(b'w')
        }
    }
}

#[near_bindgen]
impl Lottery {

    //init start lottery

    #[payable]
    pub fn enter(&mut self){
        require!(env::attached_deposit() == ONE_NEAR, "Not Engough Near Was Sent!");

        self.players.insert(env::predecessor_account_id());


        log!("{},Entered lottery", env::predecessor_account_id());

    }

    pub fn get_balance(self) -> U128 {
        near_sdk::json_types::U128(env::account_balance())
    }

    fn get_random_number(&self) -> u32 {

       let val = String::from(env::block_timestamp().to_string() + self.owner.as_str());
       let rand_num = env::keccak256_array(val.as_bytes()).into_iter().reduce(|a,b| a + b);
    
       u32::from(rand_num.unwrap())
       
    }

    fn select_winner(&mut self) -> Option<&AccountId> {

        let winner = self.get_random_number() %  self.players.len();
        let mut player = String::new();
        
        for (index, id) in self.players.into_iter().enumerate() {
            
            if index  == usize::try_from(winner).unwrap() {
                player = id.to_string();
                break
            }

        }

        self.winners.insert(self.lottery_id, AccountId::new_unchecked(player.clone()));

        log!("{} won the lottery !",player);
        
        self.players.clear();
        let winner = self.winners.get(&self.lottery_id.to_owned());
        
        self.lottery_id += 1;

        winner
        
    }

    pub fn claim(&mut self) -> Promise {

        require!(env::predecessor_account_id() == self.owner.to_owned() || self.players.contains(&env::predecessor_account_id()));

        let winner = self.select_winner().unwrap();
        
        log!("Reset counter to zero");

        Promise::new(winner.to_owned()).transfer(env::account_balance())

    }

    
    
}


#[cfg(test)]
mod tests {

    use super::*;
    use near_sdk::{testing_env, VMContext};
    use near_sdk::test_utils::VMContextBuilder;

    fn init_context() -> VMContextBuilder {
       VMContextBuilder::new()
    }

    #[test]
    fn start_lottery() {

       let contract = Lottery::default();

       let ctx = init_context();
       testing_env!(ctx.build());

       assert_eq!(contract.owner.to_string(), "bob.near".to_string(), "Contract succesfully Initailzed");

    }
    
    #[test]
    fn enter_lottery() {

       let mut ctx = init_context().attached_deposit(ONE_NEAR).to_owned();

       testing_env!(ctx.signer_account_id("test.near".parse().unwrap()).build());

       let mut contract = Lottery::default();
       contract.enter();

       assert!(contract.players.contains(&AccountId::new_unchecked("test.near".to_owned())));
  
    }

}
