#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod psp34 {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp34::extensions::{
            burnable::*,
            enumerable::*,
            mintable::*,
        },
        traits::Storage,
    };
    use lib::implem::*;
    use lib::data::Data as CustomData;
    use openbrush::contracts::ownable::*;

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        lib: CustomData,
    }

    impl PSP34 for Contract {}

    impl PSP34Mintable for Contract {}

    impl PSP34Burnable for Contract {}

    impl PSP34Enumerable for Contract {}

    impl Ownable for Contract {}

    impl CustomTrait for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._init_with_owner(instance.env().caller());
            })
        }
    }
}
