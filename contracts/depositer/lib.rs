#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
mod depositer {
    use brush::contracts::psp22::*;
    use brush::contracts::traits::psp22::*;
    use ink_lang as ink;
    use ink_prelude::vec::Vec;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Depositer {}

    impl Depositer {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn deposit_psp22(
            &mut self,
            psp22_address: AccountId,
            from: AccountId,
            amount: Balance,
        ) -> Result<(), PSP22Error> {
            PSP22Ref::transfer_from(
                &psp22_address,
                from,
                self.env().account_id(),
                amount,
                Vec::<u8>::new(),
            )
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let depositer = Depositer::default();
            assert_eq!(depositer.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut depositer = Depositer::new(false);
            assert_eq!(depositer.get(), false);
            depositer.flip();
            assert_eq!(depositer.get(), true);
        }
    }
}
