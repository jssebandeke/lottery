use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, env, require, Promise, ONE_NEAR};
use near_sdk::store::{Vector,LookupMap};
use near_sdk::json_types::U128;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Lottery {

    pub lottery_id: u32,
    pub owner: AccountId,
    pub players: Vector<AccountId>,
    pub winners: LookupMap<u32, AccountId>
    
}

impl Default for Lottery {
    fn default() -> Self {
        Self { owner: env::signer_account_id() ,
               players: Vector::new(b'i'), 
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
        require!(env::attached_deposit() > ONE_NEAR, "Not Engough Near Was Sent!");

        self.players.push(env::signer_account_id())

    }

    pub fn get_balance() -> U128 {
        near_sdk::json_types::U128(env::account_balance())
    }

    fn get_random_number(&self) -> u32 {

       let val = String::from(env::block_timestamp().to_string() + self.owner.as_str());
       let rand_num = env::keccak256_array(val.as_bytes()).into_iter().reduce(|a,b| a + b);
    
       u32::from(rand_num.unwrap())
       
    }

    fn select_winner(&mut self) -> Option<&AccountId> {

        let winner = self.get_random_number() %  self.players.len();
        let player = self.players.get(winner).expect("Array should not be empty").to_owned();

        self.winners.insert(self.lottery_id, player);
        
        self.players.clear();
        let winner = self.winners.get(&self.lottery_id.to_owned());
        
        self.lottery_id += 1;

        winner
        
    }


    pub fn claim(&mut self) -> Promise {

        require!(env::signer_account_id() == self.owner.to_owned());

        let winner = self.select_winner().unwrap();

        Promise::new(winner.to_owned()).transfer(env::account_balance())

    }

    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       
    }
}
