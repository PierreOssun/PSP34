use crate::data::Data;
pub use crate::trait_def::*;
use openbrush::traits::Storage;

impl<T: Storage<Data>> CustomTrait for T {
    default fn dummy(&mut self) -> bool {
        self.data::<Data>().dummy
    }
}