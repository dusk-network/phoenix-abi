// pub use plonk_abi::Proof;
use fermion::{self, Error};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
// TODO: this should come from `plonk_abi`
// pub const PROOF_SIZE: usize = 600;

#[derive(Clone, Copy)]
pub struct PublicKey([u8; 64]);

impl core::fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        // TODO: implement
        Ok(())
    }
}

impl Default for PublicKey {
    fn default() -> Self {
        PublicKey([0u8; 64])
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Serialize for PublicKey {
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

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PublicKeyVisitor;

        impl<'de> Visitor<'de> for PublicKeyVisitor {
            type Value = PublicKey;

            fn expecting(
                &self,
                formatter: &mut ::core::fmt::Formatter,
            ) -> ::core::fmt::Result {
                formatter.write_str("64 bytes")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<PublicKey, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut bytes = [0u8; 64];
                for (i, byte) in bytes.iter_mut().enumerate() {
                    *byte = seq.next_element()?.ok_or_else(|| {
                        serde::de::Error::invalid_length(
                            i,
                            &"expected 64 bytes",
                        )
                    })?;
                }

                Ok(PublicKey(bytes))
            }
        }

        deserializer.deserialize_tuple(64, PublicKeyVisitor)
    }
}

impl From<[u8; 64]> for PublicKey {
    fn from(arr: [u8; 64]) -> Self {
        PublicKey(arr)
    }
}

impl PublicKey {
    pub fn as_bytes(&self) -> [u8; 64] {
        self.0
    }

    // TODO: move this method as default implementation in a common trait for
    // `Note` and `Nullifier` once the following issue is fixed:
    // https://github.com/rust-lang/rust/issues/43408
    pub fn encode<T: Serialize>(t: &T) -> Result<[u8; 64], Error> {
        let mut buffer = [0u8; 64];
        fermion::encode(t, &mut buffer)?;
        Ok(buffer)
    }
}

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

            fn expecting(
                &self,
                formatter: &mut ::core::fmt::Formatter,
            ) -> ::core::fmt::Result {
                formatter.write_str("48 bytes")
            }

            fn visit_seq<A>(
                self,
                mut seq: A,
            ) -> Result<BlindingFactorBytes, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut bytes = [0u8; 48];
                for (i, byte) in bytes.iter_mut().enumerate() {
                    *byte = seq.next_element()?.ok_or_else(|| {
                        serde::de::Error::invalid_length(
                            i,
                            &"expected 48 bytes",
                        )
                    })?;
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
    value_commitment: [u8; 32],
    nonce: [u8; 24],
    r: [u8; 32],
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
    pub fn encode<T: Serialize>(
        t: &T,
    ) -> Result<[u8; Self::MAX * Self::SIZE], Error> {
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
    pub fn encode<T: Serialize>(
        t: &T,
    ) -> Result<[u8; Self::MAX * Self::SIZE], Error> {
        let mut buffer = [0u8; Self::MAX * Self::SIZE];
        fermion::encode(t, &mut buffer)?;
        Ok(buffer)
    }
}

#[cfg(feature = "std")]
mod convert {
    use super::Nullifier as ABINullifier;
    use super::PublicKey as ABIPublicKey;
    use super::{BlindingFactorBytes, Note};

    use phoenix::{
        utils, BlsScalar, Nonce, Note as NoteImpl, NoteVariant, Nullifier,
        ObfuscatedNote, PublicKey, TransactionOutput, TransparentNote,
    };

    impl From<Note> for TransactionOutput {
        fn from(item: Note) -> TransactionOutput {
            TransactionOutput::new(
                item.into(),
                0,
                BlsScalar::default(),
                PublicKey::default(),
            )
        }
    }

    impl From<Note> for NoteVariant {
        fn from(item: Note) -> Self {
            let pk_r =
                utils::deserialize_compressed_jubjub(&item.pk_r).unwrap();
            let commitment =
                utils::deserialize_bls_scalar(&item.value_commitment).unwrap();
            let nonce = Nonce::from_slice(&item.nonce).unwrap();

            if item.value == 0 {
                ObfuscatedNote::new(
                    commitment,
                    nonce,
                    utils::deserialize_compressed_jubjub(&item.r).unwrap(),
                    pk_r,
                    item.idx,
                    item.encrypted_value,
                    item.encrypted_blinding_factor.0,
                )
                .into()
            } else {
                TransparentNote::new(
                    commitment,
                    nonce,
                    utils::deserialize_compressed_jubjub(&item.r).unwrap(),
                    pk_r,
                    item.idx,
                    item.value,
                    utils::deserialize_bls_scalar(
                        &item.encrypted_blinding_factor.0,
                    )
                    .unwrap(),
                )
                .into()
            }
        }
    }

    impl From<ABINullifier> for Nullifier {
        fn from(abi_nullifier: ABINullifier) -> Self {
            Nullifier::from(
                utils::deserialize_bls_scalar(&abi_nullifier.0).unwrap(),
            )
        }
    }

    impl From<TransactionOutput> for Note {
        fn from(item: TransactionOutput) -> Self {
            let mut r_buf = [0u8; 32];
            utils::serialize_compressed_jubjub(&item.note.R(), &mut r_buf)
                .unwrap();
            let mut commitment_buf = [0u8; 32];
            utils::serialize_bls_scalar(
                item.note.value_commitment(),
                &mut commitment_buf,
            )
            .unwrap();

            let mut pk_buf = [0u8; 32];
            utils::serialize_compressed_jubjub(&item.note.pk_r(), &mut pk_buf)
                .unwrap();

            match item.note {
                NoteVariant::Transparent(note) => Note {
                    value_commitment: commitment_buf,
                    nonce: note.nonce().0,
                    r: r_buf,
                    pk_r: pk_buf,
                    idx: note.idx(),
                    value: note.value(None),
                    encrypted_value: [0u8; 24],
                    encrypted_blinding_factor: BlindingFactorBytes::from(
                        *note.encrypted_blinding_factor(),
                    ),
                },
                NoteVariant::Obfuscated(note) => Note {
                    value_commitment: commitment_buf,
                    nonce: note.nonce().0,
                    r: r_buf,
                    pk_r: pk_buf,
                    idx: note.idx(),
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
            ABINullifier(nullifier.to_bytes().unwrap())
        }
    }

    impl From<PublicKey> for ABIPublicKey {
        fn from(pk: PublicKey) -> Self {
            let mut abi_buf = [0u8; 64];
            let mut a_buf = [0u8; 32];
            utils::serialize_compressed_jubjub(&pk.A, &mut a_buf).unwrap();
            abi_buf[0..32].copy_from_slice(&a_buf);
            let mut b_buf = [0u8; 32];
            utils::serialize_compressed_jubjub(&pk.B, &mut b_buf).unwrap();
            abi_buf[32..64].copy_from_slice(&b_buf);
            ABIPublicKey(abi_buf)
        }
    }
}
