#![cfg_attr(not(feature = "std"), no_std)]
pub mod types;

pub use types::{Note, Nullifier, PublicKey};

mod external {
    extern "C" {
        pub fn phoenix_store(nullifiers: *const u8, notes: *const u8) -> bool;

        pub fn phoenix_verify(nullifiers: *const u8, notes: *const u8) -> bool;

        pub fn phoenix_credit(value: i32, pk: *const u8) -> bool;

        pub fn phoenix_is_transparent(notes: *const u8) -> bool;

        pub fn phoenix_is_addressed_to(notes: *const u8, pk: *const u8)
            -> bool;
    }
}

// TODO: fix proof
pub fn store(
    nullifiers: &[Nullifier; Nullifier::MAX],
    notes: &[Note; Note::MAX],
) -> bool {
    let nullifiers_buf =
        Nullifier::encode(nullifiers).expect("buffer insufficient");
    let notes_buf = Note::encode(notes).expect("buffer insufficient");

    unsafe {
        external::phoenix_store(nullifiers_buf.as_ptr(), notes_buf.as_ptr())
    }
}

pub fn verify(
    nullifiers: &[Nullifier; Nullifier::MAX],
    notes: &[Note; Note::MAX],
) -> bool {
    let nullifiers_buf =
        Nullifier::encode(nullifiers).expect("buffer insufficient");
    let notes_buf = Note::encode(notes).expect("buffer insufficient");

    unsafe {
        external::phoenix_verify(nullifiers_buf.as_ptr(), notes_buf.as_ptr())
    }
}

pub fn credit(value: u64, pk: &PublicKey) -> bool {
    unsafe { external::phoenix_credit(value as i32, pk.as_bytes().as_ptr()) }
}

pub fn is_transparent(notes: &[Note; Note::MAX]) -> bool {
    let notes_buf = Note::encode(notes).expect("buffer insufficient");
    unsafe { external::phoenix_is_transparent(notes_buf.as_ptr()) }
}

pub fn is_addressed_to(notes: &[Note; Note::MAX], pk: PublicKey) -> bool {
    let notes_buf = Note::encode(notes).expect("buffer insufficient");
    let pk_buf = PublicKey::encode(&pk).expect("buffer insufficient");
    unsafe {
        external::phoenix_is_addressed_to(notes_buf.as_ptr(), pk_buf.as_ptr())
    }
}
