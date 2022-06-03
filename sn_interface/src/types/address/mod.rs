// Copyright 2022 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

mod register;

#[allow(unreachable_pub)]
pub use register::RegisterAddress;

use super::{utils, Result};
use serde::{Deserialize, Serialize};
use xor_name::XorName;

/// An address of data on the network
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Debug)]
pub enum DataAddress {
    ///
    SafeKey(XorName),
    ///
    Bytes(XorName),
    ///
    Register(RegisterAddress),
}

impl DataAddress {
    /// The xorname.
    pub fn name(&self) -> &XorName {
        match self {
            Self::SafeKey(address) => address,
            Self::Bytes(address) => address,
            Self::Register(address) => address.name(),
        }
    }

    /// Returns the Address serialised and encoded in z-base-32.
    pub fn encode_to_zbase32(&self) -> Result<String> {
        utils::encode(&self)
    }

    /// Creates from z-base-32 encoded string.
    pub fn decode_from_zbase32<T: AsRef<str>>(encoded: T) -> Result<Self> {
        utils::decode(encoded)
    }

    ///
    pub fn register(name: XorName, tag: u64) -> DataAddress {
        DataAddress::Register(RegisterAddress::new(name, tag))
    }

    ///
    pub fn bytes(name: XorName) -> DataAddress {
        DataAddress::Bytes(name)
    }

    ///
    pub fn safe_key(name: XorName) -> DataAddress {
        DataAddress::SafeKey(name)
    }
}

/// An address of data on the network
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Debug)]
pub enum ReplicatedDataAddress {
    ///
    Chunk(ChunkAddress),
    ///
    Register(RegisterAddress),
}

impl ReplicatedDataAddress {
    /// The xorname.
    pub fn name(&self) -> &XorName {
        match self {
            Self::Chunk(address) => address.name(),
            Self::Register(address) => address.name(),
        }
    }

    ///
    pub fn to_replicated_address(&self) -> ReplicatedDataAddress {
        match self {
            Self::Chunk(address) => ReplicatedDataAddress::Chunk(*address),
            Self::Register(address) => ReplicatedDataAddress::Register(*address),
        }
    }
}

/// Address of a Chunk.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Debug)]
pub struct ChunkAddress(pub XorName);

impl ChunkAddress {
    /// Returns the name.
    pub fn name(&self) -> &XorName {
        &self.0
    }

    /// Returns the Address serialised and encoded in z-base-32.
    pub fn encode_to_zbase32(&self) -> Result<String> {
        utils::encode(&self)
    }

    /// Creates from z-base-32 encoded string.
    pub fn decode_from_zbase32<T: AsRef<str>>(encoded: T) -> Result<Self> {
        utils::decode(encoded)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{DataAddress, Result};

    #[test]
    fn zbase32_encode_decode_chunk_address() -> Result<()> {
        let name = xor_name::rand::random();
        let address = DataAddress::Bytes(name);
        let encoded = address.encode_to_zbase32()?;
        let decoded = DataAddress::decode_from_zbase32(&encoded)?;
        assert_eq!(address, decoded);
        Ok(())
    }
}
