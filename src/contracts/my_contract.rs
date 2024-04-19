#![cfg_attr(not(feature = "std"), no_std)]
use ink_lang as ink;

#[ink::contract]
pub mod my_contract {
    #[ink(storage)]
    pub struct MyContract {
        value: i32,
    }

    impl MyContract {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self { value: init_value }
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn increment(&mut self, by: i32) {
            self.value += by;
        }
    }
}