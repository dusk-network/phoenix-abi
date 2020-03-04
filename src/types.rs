// pub use plonk_abi::Proof;
use dusk_abi::impl_serde_for_array;
use serde::{de::Visitor, ser::SerializeTuple, Deserialize, Deserializer, Serialize, Serializer};

pub const ITEM_SIZE: usize = 305; // See `Item`

// TODO: this should come from `plonk_abi`
pub const PROOF_SIZE: usize = 600;

pub const MAX_NOTES_PER_TRANSACTION: usize = 10;

#[derive(Clone, Copy)]
struct RistrettoPointBytes([u8; 64]);

impl Default for RistrettoPointBytes {
    fn default() -> Self {
        RistrettoPointBytes([0u8; 64])
    }
}

impl_serde_for_array!(RistrettoPointBytes, 64);

impl From<[u8; 64]> for RistrettoPointBytes {
    fn from(arr: [u8; 64]) -> Self {
        RistrettoPointBytes(arr)
    }
}

#[derive(Clone, Copy)]
struct BlindingFactorBytes([u8; 48]);

impl Default for BlindingFactorBytes {
    fn default() -> Self {
        BlindingFactorBytes([0u8; 48])
    }
}

impl_serde_for_array!(BlindingFactorBytes, 48);

impl From<[u8; 48]> for BlindingFactorBytes {
    fn from(arr: [u8; 48]) -> Self {
        BlindingFactorBytes(arr)
    }
}

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct Item {
    utxo: u8,
    commitment: [u8; 32],
    nonce: [u8; 24],
    r_g: RistrettoPointBytes,
    pk_r: RistrettoPointBytes,
    idx: u64,
    value: u64,
    encrypted_value: [u8; 24],
    encrypted_blinding_factor: BlindingFactorBytes,
    nullifier: [u8; 32],
}

impl core::fmt::Debug for Item {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "a")
    }
}

#[cfg(feature = "std")]
mod convert {
    use super::Item;

    use phoenix::{
        CompressedRistretto, Nonce, NoteUtxoType, NoteVariant, Nullifier, ObfuscatedNote,
        RistrettoPoint, Scalar, TransactionItem, TransparentNote,
    };

    impl From<Item> for TransactionItem {
        fn from(item: Item) -> TransactionItem {
            let mut tx_item = TransactionItem::default();
            tx_item.set_note(item.into());

            if tx_item.utxo() == NoteUtxoType::Input {
                let nullifier =
                    Nullifier::new(Scalar::from_canonical_bytes(item.nullifier).unwrap());
                tx_item.set_nullifier(nullifier);
            }
            tx_item
        }
    }

    impl From<Item> for NoteVariant {
        fn from(item: Item) -> Self {
            // We assume that the `utxo` field is encoded as either a 1
            // or a 2
            let utxo = match item.utxo {
                1 => NoteUtxoType::Input,
                2 => NoteUtxoType::Output,
                _ => NoteUtxoType::Output, // NoteUtxoType::Unknown,
            };

            let r_g = RistrettoPoint::from_uniform_bytes(&item.r_g.0);
            let pk_r = RistrettoPoint::from_uniform_bytes(&item.pk_r.0);
            let commitment = CompressedRistretto::from_slice(&item.commitment);
            let nonce = Nonce::from_slice(&item.nonce).unwrap();

            if item.value == 0 {
                ObfuscatedNote::new(
                    utxo,
                    commitment,
                    nonce,
                    r_g,
                    pk_r,
                    item.idx.into(),
                    item.encrypted_value,
                    item.encrypted_blinding_factor.0,
                )
                .into()
            } else {
                TransparentNote::new(
                    utxo,
                    item.value,
                    nonce,
                    r_g,
                    pk_r,
                    item.idx.into(),
                    commitment,
                    item.encrypted_blinding_factor.0,
                )
                .into()
            }
        }
    }
}
