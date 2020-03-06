#![cfg_attr(not(feature = "std"), no_std)]
mod types;
pub use types::{Note, NotesBuffer, Nullifier, NullifiersBuffer};

mod external {
    use super::*;
    extern "C" {
        pub fn phoenix_store(nullifiers: &NullifiersBuffer, notes: &NotesBuffer) -> bool;

        pub fn phoenix_verify(nullifiers: &NullifiersBuffer, notes: &NotesBuffer);
    }
}

// TODO: fix proof
pub fn store(nullifiers: &NullifiersBuffer, notes: &NotesBuffer) -> bool {
    unsafe { external::phoenix_store(&nullifiers, &notes) }
}

pub fn verify(nullifiers: &NullifiersBuffer, notes: &NotesBuffer) {
    unsafe { external::phoenix_verify(&nullifiers, &notes) }
}
