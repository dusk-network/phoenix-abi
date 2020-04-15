#![cfg_attr(not(feature = "std"), no_std)]
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

// TODO: fix proof
pub fn store(
    inputs: &[Input; Input::MAX],
    notes: &[Note; Note::MAX],
    proof: &Proof,
) -> bool {
    let inputs_buf = Input::encode(inputs).expect("buffer insufficient");
    let notes_buf = Note::encode(notes).expect("buffer insufficient");
    let proof_buf = Proof::encode(proof).expect("buffer insufficient");

    unsafe {
        external::phoenix_store(
            inputs_buf.as_ptr(),
            notes_buf.as_ptr(),
            proof_buf.as_ptr(),
        )
    }
}

pub fn verify(
    inputs: &[Input; Input::MAX],
    notes: &[Note; Note::MAX],
    proof: &Proof,
) -> bool {
    let inputs_buf = Input::encode(inputs).expect("buffer insufficient");
    let notes_buf = Note::encode(notes).expect("buffer insufficient");
    let proof_buf = Proof::encode(proof).expect("buffer insufficient");

    unsafe {
        external::phoenix_verify(
            inputs_buf.as_ptr(),
            notes_buf.as_ptr(),
            proof_buf.as_ptr(),
        )
    }
}

pub fn credit(value: u64, pk: &PublicKey) -> bool {
    unsafe { external::phoenix_credit(value as i32, pk.as_bytes().as_ptr()) }
}
