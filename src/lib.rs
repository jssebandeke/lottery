use near_sdk::{near, env, AccountId, require, log, Promise, NearToken};
use near_sdk::store::{IterableSet, LookupMap};

#[near(contract_state)]
pub struct Lottery {

    pub lottery_id: u32,
    pub owner: AccountId,
    pub players: IterableSet<AccountId>,
    pub winners: LookupMap<u32, AccountId>
    
}


impl Default for Lottery {
    fn default() -> Self {
        Self { 
               owner: env::signer_account_id() ,
               players: IterableSet::new(b'i'), 
               lottery_id: Default::default(),
               winners: LookupMap::new(b'w')
        }
    }
}

#[near]
impl Lottery {

    //init start lottery

    #[payable]
    pub fn enter(&mut self){
        require!(env::attached_deposit() == NearToken::from_near(1), "Not Engough Near Was Sent!");

        self.players.insert(env::predecessor_account_id());
        self.lottery_id +=1;


        log!("{}, entered lottery {}", env::predecessor_account_id(), self.lottery_id);

    }

    pub fn get_balance(self) -> NearToken {
        env::account_balance()
    }
 
    fn get_random_number(&self) -> u32 {

       let val = String::from(env::block_timestamp().to_string() + self.owner.as_str());
       let rand_num = env::keccak256_array(val.as_bytes()).into_iter().reduce(|a,b| a + b);
    
       u32::from(rand_num.unwrap())
       
    }

    fn select_winner(&mut self) -> Option<&AccountId> {

        let winner = self.get_random_number() %  self.players.len();
        let mut player: AccountId = env::current_account_id();
        

        for (index, player_account_id) in self.players.iter().enumerate() {
            
            if index as u32  == winner {
                player = player_account_id.clone();
                break
            }

        }

        self.winners.insert(self.lottery_id, player.to_owned()); 

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

    use near_sdk::testing_env;
    use near_sdk::test_utils::VMContextBuilder;

    fn set_context() -> VMContextBuilder {
       VMContextBuilder::new()
    }

    #[test]
    fn start_lottery() {

       let contract = Lottery::default();

       let ctx = set_context();
       testing_env!(ctx.build());

       assert_eq!(contract.owner.to_string(), "bob.near".to_string(), "Contract succesfully Initailzed");
       log!("The contract owner is {}",contract.owner);

    }


    #[test]
    fn enter_lottery() {

        let mut contract = Lottery::default();

        let mut ctx = set_context();
        ctx.predecessor_account_id("player.near".parse().unwrap());
        ctx.attached_deposit(NearToken::from_near(1));
        
        testing_env!(ctx.build());
        contract.enter();

        //let player: AccountId = "test.near".parse().unwrap();
 
        contract.players.iter().for_each(|player| log!("{}",player));
 
        assert!(contract.players.len() == 1);
   
     }
 
 

}



