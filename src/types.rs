// pub use plonk_abi::Proof;
use fermion::{self, Error};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
// TODO: this should come from `plonk_abi`
// pub const PROOF_SIZE: usize = 600;

#[derive(Clone, Copy)]
pub struct BlindingFactorBytes([u8; 48]);

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
    r_g: [u8; 32],
    pk_r: [u8; 32],
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
    use super::Nullifier as ABINullifier;
    use super::{BlindingFactorBytes, Note};

    use phoenix::{
        CompressedRistretto, Nonce, Note as NoteImpl, NoteUtxoType, NoteVariant, Nullifier,
        ObfuscatedNote, Scalar, TransactionItem, TransparentNote,
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

            let r_g = CompressedRistretto::from_slice(&item.r_g);
            let pk_r = CompressedRistretto::from_slice(&item.pk_r);
            let commitment = CompressedRistretto::from_slice(&item.commitment);
            let nonce = Nonce::from_slice(&item.nonce).unwrap();

            if item.value == 0 {
                ObfuscatedNote::new(
                    utxo,
                    commitment,
                    nonce,
                    r_g.decompress().unwrap(),
                    pk_r.decompress().unwrap(),
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
                    r_g.decompress().unwrap(),
                    pk_r.decompress().unwrap(),
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

    impl From<TransactionItem> for Note {
        fn from(item: TransactionItem) -> Self {
            match item.note() {
                NoteVariant::Transparent(note) => Note {
                    utxo: 1,
                    commitment: note.commitment().to_bytes(),
                    nonce: note.nonce().0,
                    r_g: note.r_g().compress().to_bytes(),
                    pk_r: note.pk_r().compress().to_bytes(),
                    idx: note.idx().pos,
                    value: note.value(None),
                    encrypted_value: [0u8; 24],
                    encrypted_blinding_factor: BlindingFactorBytes::from(
                        *note.encrypted_blinding_factor(),
                    ),
                },
                NoteVariant::Obfuscated(note) => Note {
                    utxo: 1,
                    commitment: note.commitment().to_bytes(),
                    nonce: note.nonce().0,
                    r_g: note.r_g().compress().to_bytes(),
                    pk_r: note.pk_r().compress().to_bytes(),
                    idx: note.idx().pos,
                    value: 0,
                    encrypted_value: *note.encrypted_value().unwrap(),
                    encrypted_blinding_factor: BlindingFactorBytes::from(
                        *note.encrypted_blinding_factor(),
                    ),
                },
            }
        }
    }

    impl From<Nullifier> for ABINullifier {
        fn from(nullifier: Nullifier) -> Self {
            ABINullifier(nullifier.point().to_bytes())
        }
    }
}
