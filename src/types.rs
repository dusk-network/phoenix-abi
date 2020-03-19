// pub use plonk_abi::Proof;
use fermion::{self, Error};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
// TODO: this should come from `plonk_abi`
// pub const PROOF_SIZE: usize = 600;

#[derive(Clone, Copy)]
struct RistrettoPointBytes([u8; 64]);

impl Default for RistrettoPointBytes {
    fn default() -> Self {
        RistrettoPointBytes([0u8; 64])
    }
}

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
                for (i, byte) in bytes.iter_mut().enumerate() {
                    *byte = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(i, &"expected 64 bytes"))?;
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

impl Default for BlindingFactorBytes {
    fn default() -> Self {
        BlindingFactorBytes([0u8; 48])
    }
}

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
                for (i, byte) in bytes.iter_mut().enumerate() {
                    *byte = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(i, &"expected 48 bytes"))?;
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

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct Note {
    utxo: u8,
    commitment: [u8; 32],
    nonce: [u8; 24],
    r_g: RistrettoPointBytes,
    pk_r: RistrettoPointBytes,
    idx: u64,
    value: u64,
    encrypted_value: [u8; 24],
    encrypted_blinding_factor: BlindingFactorBytes,
}

impl Note {
    pub const MAX: usize = 10;
    pub const SIZE: usize = 273;

    // TODO: move this method as default implementation in a common trait for
    // `Note` and `Nullifier` once the following issue is fixed:
    // https://github.com/rust-lang/rust/issues/43408
    pub fn encode<T: Serialize>(t: &T) -> Result<[u8; Self::MAX * Self::SIZE], Error> {
        let mut buffer = [0u8; Self::MAX * Self::SIZE];
        fermion::encode(t, &mut buffer)?;
        Ok(buffer)
    }
}

impl core::fmt::Debug for Note {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "a")
    }
}

#[derive(Clone, Copy, Default, Serialize, Deserialize, Debug)]
pub struct Nullifier([u8; Nullifier::SIZE]);

impl Nullifier {
    pub const MAX: usize = 8;
    pub const SIZE: usize = 32;

    // TODO: move this method as default implementation in a common trait for
    // `Note` and `Nullifier` once the following issue is fixed:
    // https://github.com/rust-lang/rust/issues/43408
    pub fn encode<T: Serialize>(t: &T) -> Result<[u8; Self::MAX * Self::SIZE], Error> {
        let mut buffer = [0u8; Self::MAX * Self::SIZE];
        fermion::encode(t, &mut buffer)?;
        Ok(buffer)
    }
}

#[cfg(feature = "std")]
mod convert {
    use super::Note;
    use super::Nullifier as ABINullifier;

    use phoenix::{
        CompressedRistretto, Nonce, NoteUtxoType, NoteVariant, Nullifier, ObfuscatedNote,
        RistrettoPoint, Scalar, TransactionItem, TransparentNote,
    };

    impl From<Note> for TransactionItem {
        fn from(item: Note) -> TransactionItem {
            let mut tx_item = TransactionItem::default();
            tx_item.set_note(item.into());
            tx_item
        }
    }

    impl From<Note> for NoteVariant {
        fn from(item: Note) -> Self {
            // Should always be an output note
            let utxo = NoteUtxoType::Output;

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

    impl From<ABINullifier> for Nullifier {
        fn from(abi_nullifier: ABINullifier) -> Self {
            Nullifier::new(Scalar::from_canonical_bytes(abi_nullifier.0).unwrap())
        }
    }
}
