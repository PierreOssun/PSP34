use crate::data::Data;
pub use crate::trait_def::*;
use openbrush::{
    contracts::{
        psp34::extensions::enumerable::*,
    },
    traits::Storage,
};
use openbrush::contracts::psp34::Id;


impl<T> CustomTrait for T
    where
        T: Storage<Data>
        + Storage<psp34::Data<enumerable::Balances>>
{
    default fn dummy(&mut self) -> Result<(), PSP34Error> {
        self.data::<psp34::Data<enumerable::Balances>>()
            .owner_of(Id::U8(0))
            .ok_or(PSP34Error::TokenNotExists)?;
        Ok(())
    }
}