// pub use plonk_abi::Proof;
use fermion::{self, Error};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
// TODO: this should come from `plonk_abi`

#[derive(Clone, Copy)]
pub struct Proof(pub [u8; Proof::SIZE]);

impl Default for Proof {
    fn default() -> Self {
        Proof([0u8; Proof::SIZE])
    }
}

impl AsRef<[u8]> for Proof {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Serialize for Proof {
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

impl<'de> Deserialize<'de> for Proof {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProofVisitor;

        impl<'de> Visitor<'de> for ProofVisitor {
            type Value = Proof;

            fn expecting(
                &self,
                formatter: &mut ::core::fmt::Formatter,
            ) -> ::core::fmt::Result {
                formatter.write_str("1097 bytes")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Proof, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut bytes = [0u8; 1097];
                for (i, byte) in bytes.iter_mut().enumerate() {
                    *byte = seq.next_element()?.ok_or_else(|| {
                        serde::de::Error::invalid_length(
                            i,
                            &"expected 1097 bytes",
                        )
                    })?;
                }

                Ok(Proof(bytes))
            }
        }

        deserializer.deserialize_tuple(1097, ProofVisitor)
    }
}

impl core::fmt::Debug for Proof {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        // TODO: implement
        Ok(())
    }
}

impl Proof {
    pub const SIZE: usize = 1097;

    pub fn encode<T: Serialize>(t: &T) -> Result<[u8; Proof::SIZE], Error> {
        let mut buffer = [0u8; Proof::SIZE];
        fermion::encode(t, &mut buffer)?;
        Ok(buffer)
    }
}

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
    blinding_factor: [u8; 32],
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
    use std::convert::TryFrom;

    use phoenix::{
        rpc, utils, BlsScalar, Error, Nonce, Note as NoteImpl, NoteVariant,
        Nullifier, ObfuscatedNote, PublicKey, TransactionOutput,
        TransparentNote,
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
                    utils::deserialize_bls_scalar(&item.blinding_factor)
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

    impl TryFrom<&rpc::Nullifier> for ABINullifier {
        type Error = Error;

        fn try_from(nullifier: &rpc::Nullifier) -> Result<Self, Error> {
            let mut scalar_buf = [0u8; 32];
            let h = nullifier.h.as_ref().ok_or_else(|| Error::Generic)?;
            scalar_buf.copy_from_slice(&h.data);
            Ok(ABINullifier(scalar_buf))
        }
    }

    impl TryFrom<&rpc::TransactionOutput> for Note {
        type Error = Error;

        fn try_from(output: &rpc::TransactionOutput) -> Result<Self, Error> {
            let note = output.note.as_ref().ok_or(Error::Generic)?;

            let value_commitment =
                note.value_commitment.as_ref().ok_or(Error::Generic)?;
            let mut value_commitment_buf = [0u8; 32];
            value_commitment_buf.copy_from_slice(&value_commitment.data);

            let nonce = note.nonce.as_ref().ok_or(Error::Generic)?;
            let mut nonce_buf = [0u8; 24];
            nonce_buf.copy_from_slice(&nonce.bs);

            let r = note.r_g.as_ref().ok_or(Error::Generic)?;
            let mut r_buf = [0u8; 32];
            r_buf.copy_from_slice(&r.y);

            let pk_r = note.pk_r.as_ref().ok_or(Error::Generic)?;
            let mut pk_r_buf = [0u8; 32];
            pk_r_buf.copy_from_slice(&pk_r.y);

            let mut abi_note = Note {
                value_commitment: value_commitment_buf,
                nonce: nonce_buf,
                r: r_buf,
                pk_r: pk_r_buf,
                idx: note.pos,
                value: 0,
                encrypted_value: [0u8; 24],
                blinding_factor: [0u8; 32],
                encrypted_blinding_factor: BlindingFactorBytes::default(),
            };

            let blinding_factor =
                note.blinding_factor.as_ref().ok_or(Error::Generic)?;
            match blinding_factor {
                rpc::note::BlindingFactor::TransparentBlindingFactor(
                    scalar,
                ) => {
                    abi_note.blinding_factor.copy_from_slice(&scalar.data);
                }
                rpc::note::BlindingFactor::EncryptedBlindingFactor(bytes) => {
                    let mut encrypted_blinding_factor_buf = [0u8; 48];
                    encrypted_blinding_factor_buf.copy_from_slice(&bytes);
                    abi_note.encrypted_blinding_factor.0 =
                        encrypted_blinding_factor_buf;
                }
            }

            let value = note.value.as_ref().ok_or(Error::Generic)?;
            match value {
                rpc::note::Value::TransparentValue(num) => {
                    abi_note.value = *num;
                }
                rpc::note::Value::EncryptedValue(bytes) => {
                    let mut encrypted_value_buf = [0u8; 24];
                    encrypted_value_buf.copy_from_slice(&bytes);
                    abi_note.encrypted_value = encrypted_value_buf;
                }
            }

            Ok(abi_note)
        }
    }
}
