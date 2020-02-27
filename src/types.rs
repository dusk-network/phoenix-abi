// pub use plonk_abi::Proof;
use phoenix::{
    CompressedRistretto, Nonce, NoteUtxoType, NoteVariant, Nullifier, ObfuscatedNote,
    RistrettoPoint, Scalar, TransactionItem, TransparentNote,
};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub const ITEM_SIZE: usize = 305; // See `Item`

// TODO: this should come from `plonk_abi`
pub const PROOF_SIZE: usize = 600;

#[derive(Clone, Copy)]
struct RistrettoPointBytes([u8; 64]);

impl Serialize for RistrettoPointBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeTuple;
        let mut seq = serializer.serialize_tuple(self.0.len())?;
        for byte in self.0.iter() {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for RistrettoPointBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RistrettoPointBytesVisitor;

        impl<'de> Visitor<'de> for RistrettoPointBytesVisitor {
            type Value = RistrettoPointBytes;

            fn expecting(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                formatter.write_str("64 bytes")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<RistrettoPointBytes, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut bytes = [0u8; 64];
                for i in 0..64 {
                    bytes[i] = seq
                        .next_element()?
                        .ok_or(serde::de::Error::invalid_length(i, &"expected 64 bytes"))?;
                }

                Ok(RistrettoPointBytes(bytes))
            }
        }

        deserializer.deserialize_tuple(64, RistrettoPointBytesVisitor)
    }
}

impl From<[u8; 64]> for RistrettoPointBytes {
    fn from(arr: [u8; 64]) -> Self {
        RistrettoPointBytes(arr)
    }
}

#[derive(Clone, Copy)]
struct BlindingFactorBytes([u8; 48]);

impl Serialize for BlindingFactorBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeTuple;
        let mut seq = serializer.serialize_tuple(self.0.len())?;
        for byte in self.0.iter() {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for BlindingFactorBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BlindingFactorBytesVisitor;

        impl<'de> Visitor<'de> for BlindingFactorBytesVisitor {
            type Value = BlindingFactorBytes;

            fn expecting(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                formatter.write_str("48 bytes")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<BlindingFactorBytes, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut bytes = [0u8; 48];
                for i in 0..48 {
                    bytes[i] = seq
                        .next_element()?
                        .ok_or(serde::de::Error::invalid_length(i, &"expected 48 bytes"))?;
                }

                Ok(BlindingFactorBytes(bytes))
            }
        }

        deserializer.deserialize_tuple(48, BlindingFactorBytesVisitor)
    }
}

impl From<[u8; 48]> for BlindingFactorBytes {
    fn from(arr: [u8; 48]) -> Self {
        BlindingFactorBytes(arr)
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
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

impl From<Item> for TransactionItem {
    fn from(item: Item) -> TransactionItem {
        let mut tx_item = TransactionItem::default();
        tx_item.set_note(item.into());

        if tx_item.utxo() == NoteUtxoType::Input {
            let nullifier = Nullifier::new(Scalar::from_canonical_bytes(item.nullifier).unwrap());
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
            _ => panic!("oh bother"), // NoteUtxoType::Unknown,
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

// pub struct EncodedNote([u8; ITEM_SIZE]);
//
// // TODO: impl serde
// impl EncodedNote {
//     pub fn as_array(&self) -> [u8; ITEM_SIZE] {
//         self.0
//     }
// }
