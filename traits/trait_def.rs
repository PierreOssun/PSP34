use crate::data::Data;
use openbrush::traits::Storage;

#[openbrush::trait_definition]
pub trait CustomTrait: Storage<Data> {
    #[ink(message)]
    fn dummy(&mut self) -> bool;
}