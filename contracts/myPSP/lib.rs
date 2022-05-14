#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)] //false positive - without this attribute contract does not compile

#[brush::contract]
pub mod my_psp22 {

    use brush::contracts::psp22::*;

    use ink_lang::codegen::EmitEvent;
    use ink_lang::codegen::Env;
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(Default, PSP22Storage, SpreadAllocate)]
    pub struct MyPsp22 {
        #[PSP22StorageField]
        psp22: PSP22Data,
    }

    impl PSP22 for MyPsp22 {}

    impl MyPsp22 {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let caller = instance.env().caller();
                instance
                    .psp22
                    .balances
                    .insert(&caller, &(10000000000000000000000000));
            })
        }

        // #[ink(message)]
        // pub fn mint_any_caller(&mut self, amount: Balance) -> Result<(), PSP22Error> {
        //     let to = self.env().caller();
        //     self._mint(to, amount)
        // }
    }
}
