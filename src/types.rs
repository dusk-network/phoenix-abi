use dataview::Pod;
// TODO: this should come from `plonk_abi`
/// A serialized PLONK proof.
#[repr(C)]
#[derive(Clone, Copy, Pod)]
pub struct Proof {
    bytes: [u8; Proof::SIZE],
}

impl Default for Proof {
    fn default() -> Self {
        Proof {
            bytes: [0u8; Proof::SIZE],
        }
    }
}
impl AsRef<[u8]> for Proof {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl core::fmt::Debug for Proof {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Proof(")?;
        for i in 0..Proof::SIZE {
            write!(f, "{:02x}", self.bytes[i])?;
        }
        write!(f, ")")
    }
}

impl Proof {
    /// Size of an encoded [`Proof`] in bytes.
    pub const SIZE: usize = 1097;

    /// Builds a [`Proof`] from an array of bytes.
    pub fn from_bytes(bytes: [u8; Proof::SIZE]) -> Self {
        Proof { bytes }
    }

    /// Returns the [`Proof`] as an array of bytes.
    pub fn to_bytes(&self) -> [u8; Proof::SIZE] {
        self.bytes
    }
}

/// A Phoenix public key, represented as a 64-byte array.
/// The 64-byte array holds two scalars, both with a size of 32 bytes.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PublicKey([u8; 64]);

unsafe impl Pod for PublicKey {}

impl Default for PublicKey {
    fn default() -> Self {
        PublicKey([0u8; 64])
    }
}

impl core::fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "PublicKey(")?;
        for i in 0..64 {
            write!(f, "{:02x}", self.0[i])?;
        }
        write!(f, ")")
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 64]> for PublicKey {
    fn from(arr: [u8; 64]) -> Self {
        PublicKey(arr)
    }
}

impl PublicKey {
    /// Returns the [`PublicKey`] as an array of bytes.
    pub fn as_bytes(&self) -> [u8; 64] {
        self.0
    }
}

/// A byte array representing the blinding factor found on a Phoenix note.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BlindingFactorBytes([u8; 48]);

unsafe impl Pod for BlindingFactorBytes {}

impl Default for BlindingFactorBytes {
    fn default() -> Self {
        BlindingFactorBytes([0u8; 48])
    }
}

impl From<[u8; 48]> for BlindingFactorBytes {
    fn from(arr: [u8; 48]) -> Self {
        BlindingFactorBytes(arr)
    }
}

impl core::fmt::Debug for BlindingFactorBytes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "BlindingFactorBytes(")?;
        for i in 0..48 {
            write!(f, "{:02x}", self.0[i])?;
        }
        write!(f, ")")
    }
}

/// A Phoenix transaction output, consisting of all the fields found on a
/// Phoenix note. The structure makes no distinction between transparent and
/// obfuscated notes, and thus contains the necessary fields for both types
/// to be represented by the [`Note`].
/// N.B. this may mean some fields are empty, even though the note was converted
/// correctly.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod)]
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
    /// Maximum amount of [`Note`]s in a Phoenix transaction.
    pub const MAX: usize = 3;
    /// Total size of an encoded [`Note`] in bytes.
    pub const SIZE: usize = 240;
}

/// A Phoenix transaction input, consisting of a nullifier and a merkle root.
/// This only contains the non-sensitive information of a Phoenix input,
/// and should correspond to what we receive from the wire.
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, Pod)]
pub struct Input {
    nullifier: [u8; 32],
    merkle_root: [u8; 32],
}

impl Input {
    /// Maximum amount of [`Input`]s in a Phoenix transaction.
    pub const MAX: usize = 1;
    /// Total size of an encoded [`Input`] in bytes.
    pub const SIZE: usize = 64;
}

#[cfg(feature = "std")]
mod convert {
    use super::Input as ABIInput;
    use super::{BlindingFactorBytes, Note};
    use std::convert::{TryFrom, TryInto};

    use phoenix::{
        rpc, utils, BlsScalar, Error, Nonce, NoteVariant, Nullifier,
        ObfuscatedNote, PublicKey, TransactionInput, TransactionOutput,
        TransparentNote,
    };

    impl TryFrom<Note> for TransactionOutput {
        type Error = Error;

        fn try_from(item: Note) -> Result<TransactionOutput, Error> {
            Ok(TransactionOutput::new(
                item.try_into()?,
                0,
                BlsScalar::default(),
                PublicKey::default(),
            ))
        }
    }

    impl TryFrom<Note> for NoteVariant {
        type Error = Error;

        fn try_from(item: Note) -> Result<Self, Error> {
            let pk_r = utils::deserialize_compressed_jubjub(&item.pk_r)?;
            let commitment =
                utils::deserialize_bls_scalar(&item.value_commitment)?;
            let nonce = Nonce::from_slice(&item.nonce).ok_or(Error::Generic)?;

            if item.value == 0 {
                Ok(ObfuscatedNote::new(
                    commitment,
                    nonce,
                    utils::deserialize_compressed_jubjub(&item.r)?,
                    pk_r,
                    item.idx,
                    item.encrypted_value,
                    item.encrypted_blinding_factor.0,
                )
                .into())
            } else {
                Ok(TransparentNote::new(
                    commitment,
                    nonce,
                    utils::deserialize_compressed_jubjub(&item.r)?,
                    pk_r,
                    item.idx,
                    item.value,
                    utils::deserialize_bls_scalar(&item.blinding_factor)?,
                )
                .into())
            }
        }
    }

    impl TryFrom<ABIInput> for TransactionInput {
        type Error = Error;

        fn try_from(abi_input: ABIInput) -> Result<Self, Error> {
            let mut input = TransactionInput::default();
            input.nullifier = Nullifier::from(utils::deserialize_bls_scalar(
                &abi_input.nullifier,
            )?);
            input.merkle_root =
                utils::deserialize_bls_scalar(&abi_input.merkle_root)?;
            Ok(input)
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
        use phoenix::{crypto, Note as NoteImpl, NoteGenerator, SecretKey};

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

            let abi_output = Note::try_from(&rpc_output);

            assert!(abi_output.is_ok())
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

            let abi_input = ABIInput::try_from(&rpc_input);

            assert!(abi_input.is_ok())
        }
    }
}
