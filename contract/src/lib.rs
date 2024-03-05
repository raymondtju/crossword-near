use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
const PUZZLE_NUMBER: u8 = 1;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    crossword_solution: String,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(solution: String) -> Self {
        Self {
            crossword_solution: solution,
        }
    }

    pub fn get_puzzle_number(&self) -> u8 {
        PUZZLE_NUMBER
    }

    // Comment it so setter was not public
    // fn set_solution(&mut self, solution: String) {
    //     self.crossword_solution = solution
    // }

    pub fn guess_solution(&mut self, solution: String) -> bool {
        let hashed_input = env::sha256(solution.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input);

        if hashed_input_hex == self.crossword_solution {
            env::log_str("Clever, you guessed right");
            true
        } else {
            env::log_str("Tray again lah");
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hex::encode;
    use near_sdk::{
        env::sha256,
        test_utils::{get_logs, VMContextBuilder},
        testing_env, AccountId,
    };
    use tokio::test;

    #[test]
    async fn debug_get_hash() {
        testing_env!(VMContextBuilder::new().build());

        let debug_solution = "bah ngakak";
        let debug_hash_bytes = sha256(debug_solution.as_bytes());
        println!("bytes was: {:?}", debug_hash_bytes);
        let debug_hash_string = encode(debug_hash_bytes);
        println!("hash was: {:?}", debug_hash_string);
    }

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[tokio::test]
    async fn check_guess_solution() {
        // let acc = AccountId::new_unchecked("crossword.ferro.near".to_string());
        // let context = get_context(acc);
        // testing_env!(context.build());
        testing_env!(VMContextBuilder::new().build());

        let mut contract = Contract::new(
            "3f1eced51bac0cb5351bb7c40eacb93146935a0490e7e2d24d177789a3a76e31".to_string(),
        );
        // let mut contract = Contract::default();
        // contract.set_solution(
        //      "3f1eced51bac0cb5351bb7c40eacb93146935a0490e7e2d24d177789a3a76e31".to_string(),
        // );

        let mut guess_result = contract.guess_solution("hehehhe".to_string());
        assert!(!guess_result, "Expect the wrong answer");
        assert_eq!(get_logs(), ["Tray again lah"], "Expected a failure log.");

        guess_result = contract.guess_solution("bah ngakak".to_string());
        println!("{:?}", contract.crossword_solution);
        assert!(guess_result, "Expected the correct answer");
        assert_eq!(
            get_logs(),
            ["Tray again lah", "Clever, you guessed right"],
            "Expected a successful log after the previous failed log."
        );
    }
}

// // Define the default message
// const DEFAULT_MESSAGE: &str = "Hello";

// // Define the contract structure
// #[near_bindgen]
// #[derive(BorshDeserialize, BorshSerialize)]
// pub struct Contract {
//     message: String,
// }

// // Define the default, which automatically initializes the contract
// impl Default for Contract{
//     fn default() -> Self{
//         Self{message: DEFAULT_MESSAGE.to_string()}
//     }
// }

// // Implement the contract structure
// #[near_bindgen]
// impl Contract {
//     // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
//     pub fn get_greeting(&self) -> String {
//         return self.message.clone();
//     }

//     // Public method - accepts a greeting, such as "howdy", and records it
//     pub fn set_greeting(&mut self, message: String) {
//         // Use env::log to record logs permanently to the blockchain!
//         log!("Saving greeting {}", message);
//         self.message = message;
//     }
// }

// /*
//  * The rest of this file holds the inline tests for the code above
//  * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
//  */
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn get_default_greeting() {
//         let contract = Contract::default();
//         // this test did not call set_greeting so should return the default "Hello" greeting
//         assert_eq!(
//             contract.get_greeting(),
//             "Hello".to_string()
//         );
//     }

//     #[test]
//     fn set_then_get_greeting() {
//         let mut contract = Contract::default();
//         contract.set_greeting("howdy".to_string());
//         assert_eq!(
//             contract.get_greeting(),
//             "howdy".to_string()
//         );
//     }
// }
