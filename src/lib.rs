#![cfg_attr(not(feature = "std"), no_std)]
mod types;
pub use types::{
    Note, NotesBuffer, Nullifier, NullifiersBuffer, MAX_NOTES_PER_TRANSACTION,
    MAX_NULLIFIERS_PER_TRANSACTION, NOTE_SIZE, NULLIFIER_SIZE, PROOF_SIZE,
};

mod external {
    use super::*;
    extern "C" {
        pub fn phoenix_store(nullifiers: &NullifiersBuffer, notes: &NotesBuffer) -> bool;
    }
}

// TODO: fix proof
pub fn store(nullifiers: &NullifiersBuffer, notes: &NotesBuffer) -> bool {
    unsafe { external::phoenix_store(&nullifiers, &notes) }
}
