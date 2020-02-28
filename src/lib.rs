#![cfg_attr(not(feature = "std"), no_std)]
mod types;
pub use types::{Item, ITEM_SIZE, MAX_NOTES_PER_TRANSACTION, PROOF_SIZE};

mod external {
    use super::*;
    extern "C" {
        pub fn phoenix_store(items_buf: &[u8; MAX_NOTES_PER_TRANSACTION * ITEM_SIZE]) -> bool;
    }
}

// TODO: fix proof
pub fn store(items: &[u8; MAX_NOTES_PER_TRANSACTION * ITEM_SIZE]) -> bool {
    unsafe { external::phoenix_store(&items) }
}
