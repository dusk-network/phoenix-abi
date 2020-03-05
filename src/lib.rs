#![cfg_attr(not(feature = "std"), no_std)]
mod types;
pub use types::{
    Note, Nullifier, MAX_NOTES_PER_TRANSACTION, MAX_NULLIFIERS_PER_TRANSACTION, NOTE_SIZE,
    NULLIFIER_SIZE, PROOF_SIZE,
};

mod external {
    use super::*;
    extern "C" {
        pub fn phoenix_store(
            nullifiers_buf: &[u8; MAX_NULLIFIERS_PER_TRANSACTION * NULLIFIER_SIZE],
            notes_buf: &[u8; MAX_NOTES_PER_TRANSACTION * NOTE_SIZE],
        ) -> bool;
    }
}

// TODO: fix proof
pub fn store(
    nullifiers: &[u8; MAX_NULLIFIERS_PER_TRANSACTION * NULLIFIER_SIZE],
    notes: &[u8; MAX_NOTES_PER_TRANSACTION * NOTE_SIZE],
) -> bool {
    unsafe { external::phoenix_store(&nullifiers, &notes) }
}
