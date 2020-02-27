#![no_std]
mod types;
use phoenix::MAX_NOTES_PER_TRANSACTION;
pub use types::{Item, ITEM_SIZE, PROOF_SIZE};

mod external {
    use super::*;
    extern "C" {
        pub fn phoenix_store(
            items_buf: &[u8; MAX_NOTES_PER_TRANSACTION * ITEM_SIZE],
            proof_buf: &[u8; PROOF_SIZE],
        ) -> bool;
    }
}

// TODO: fix proof
pub fn store(items: &[u8; MAX_NOTES_PER_TRANSACTION * ITEM_SIZE], proof: u8) -> bool {
    unsafe { external::phoenix_store(&items, &[0u8; PROOF_SIZE]) }
}
