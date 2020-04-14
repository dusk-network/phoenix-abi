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
                let mut bytes = [0u8; Proof::SIZE];
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

        deserializer.deserialize_tuple(Proof::SIZE, ProofVisitor)
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
    pub const MAX: usize = 3;
    pub const SIZE: usize = 240;

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
pub struct Input {
    nullifier: [u8; 32],
    merkle_root: [u8; 32],
}

impl Input {
    pub const MAX: usize = 1;
    pub const SIZE: usize = 64;

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
    use super::Input as ABIInput;
    use super::PublicKey as ABIPublicKey;
    use super::{BlindingFactorBytes, Note};
    use std::convert::TryFrom;

    use phoenix::{
        rpc, utils, BlsScalar, Error, Nonce, Note as NoteImpl, NoteVariant,
        Nullifier, ObfuscatedNote, PublicKey, TransactionInput,
        TransactionOutput, TransparentNote,
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

    impl From<ABIInput> for TransactionInput {
        fn from(abi_input: ABIInput) -> Self {
            let mut input = TransactionInput::default();
            input.nullifier = Nullifier::from(
                utils::deserialize_bls_scalar(&abi_input.nullifier).unwrap(),
            );
            input.merkle_root =
                utils::deserialize_bls_scalar(&abi_input.merkle_root).unwrap();
            input
        }
    }

    impl TryFrom<&rpc::TransactionInput> for ABIInput {
        type Error = Error;

        fn try_from(input: &rpc::TransactionInput) -> Result<Self, Error> {
            let mut nullifier_buf = [0u8; 32];
            let h = input
                .nullifier
                .as_ref()
                .ok_or(Error::Generic)?
                .h
                .as_ref()
                .ok_or_else(|| Error::Generic)?;
            nullifier_buf.copy_from_slice(&h.data);
            let mut merkle_root_buf = [0u8; 32];
            let h = input.merkle_root.as_ref().ok_or(Error::Generic)?;
            merkle_root_buf.copy_from_slice(&h.data);
            Ok(ABIInput {
                nullifier: nullifier_buf,
                merkle_root: merkle_root_buf,
            })
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

    #[cfg(test)]
    mod test {
        use super::*;
        use phoenix::{crypto, NoteGenerator, SecretKey};

        #[test]
        fn convert_output_to_note() {
            // Mandatory Phoenix init stuff
            utils::init();

            // First, let's make an actual phoenix tx output, and convert that to
            // an RPC one.
            let sk = SecretKey::default();
            let pk = sk.public_key();
            let value = 95;
            let (note, blinding_factor) = TransparentNote::output(&pk, value);
            let output = note.to_transaction_output(value, blinding_factor, pk);

            let rpc_output: rpc::TransactionOutput = output.into();

            let abi_output = Note::try_from(&rpc_output).unwrap();
        }

        #[test]
        fn convert_input() {
            // Mandatory Phoenix init stuff
            utils::init();

            // Create an actual input first, and then cast it to an RPC one.
            let sk = SecretKey::default();
            let pk = sk.public_key();
            let value = 100;
            let note = TransparentNote::output(&pk, value).0;
            let merkle_opening = crypto::MerkleProof::mock(note.hash());
            let input = note.to_transaction_input(merkle_opening, sk);

            let rpc_input: rpc::TransactionInput = input.into();

            let abi_input = ABIInput::try_from(&rpc_input).unwrap();
        }
    }
}
