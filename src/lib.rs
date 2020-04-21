//! The `phoenix-abi` crate contains the ABI to enable phoenix's call  from
//! Smart Contract running in the Rusk VM

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

/// A collection of different Phoenix data structures, in ABI form.
pub mod types;

pub use types::{Input, Note, Proof, PublicKey};

mod external {
    extern "C" {
        pub fn phoenix_store(
            nullifiers: *const u8,
            notes: *const u8,
            proof: *const u8,
        ) -> bool;

        pub fn phoenix_verify(
            nullifiers: *const u8,
            notes: *const u8,
            proof: *const u8,
        ) -> bool;

        pub fn phoenix_credit(value: i32, pk: *const u8) -> bool;
    }
}

/// Stores a phoenix transaction re-constructed from the inputs, notes and proof
/// given.
///
/// Returns `true` if the transaction is successfully stored, `false` otherwise.
// TODO: fix proof
pub fn store(
    inputs: &[Input; Input::MAX],
    notes: &[Note; Note::MAX],
    proof: &Proof,
) -> bool {
    unsafe {
        external::phoenix_store(
            inputs.as_ptr() as *const u8,
            notes.as_ptr() as *const u8,
            proof.as_ref().as_ptr(),
        )
    }
}

/// Verifies a phoenix transaction re-constructed from the inputs, notes and
/// proof given.
///
/// Rurns `true` if the transaction is correct, `false` otherwise.
pub fn verify(
    inputs: &[Input; Input::MAX],
    notes: &[Note; Note::MAX],
    proof: &Proof,
) -> bool {
    unsafe {
        external::phoenix_verify(
            inputs.as_ptr() as *const u8,
            notes.as_ptr() as *const u8,
            proof.as_ref().as_ptr(),
        )
    }
}

/// Credits to the recipient (`PublicKey`) minted DUSK from new phoenix
/// transaction.
pub fn credit(value: u64, pk: &PublicKey) -> bool {
    unsafe { external::phoenix_credit(value as i32, pk.as_bytes().as_ptr()) }
}
