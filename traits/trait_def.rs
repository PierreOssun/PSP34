use openbrush::{
    contracts::psp34::PSP34Error,
};

#[openbrush::trait_definition]
pub trait CustomTrait {
    #[ink(message, payable)]
    fn dummy(&mut self) -> Result<(), PSP34Error>;
}