#[cfg(feature = "le-stream")]
use le_stream::derive::{FromLeStream, ToLeStream};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

mod ember {
    pub type Eui64 = u64;
    pub type KeyStructBitmask = u16;
}
pub type ManKey = [u8; 16];

#[cfg_attr(feature = "le-stream", derive(FromLeStream, ToLeStream))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManContext {
    core_key_type: ManKey,
    key_index: u8,
    derived_type: u8,
    eui64: ember::Eui64,
    multi_network_index: u8,
    flags: u8,
    psa_key_alg_permission: u32,
}

impl ManContext {
    #[must_use]
    pub const fn new(
        core_key_type: ManKey,
        key_index: u8,
        derived_type: u8,
        eui64: ember::Eui64,
        multi_network_index: u8,
        flags: u8,
        psa_key_alg_permission: u32,
    ) -> Self {
        Self {
            core_key_type,
            key_index,
            derived_type,
            eui64,
            multi_network_index,
            flags,
            psa_key_alg_permission,
        }
    }

    #[must_use]
    pub const fn core_key_type(&self) -> &ManKey {
        &self.core_key_type
    }

    #[must_use]
    pub const fn key_index(&self) -> u8 {
        self.key_index
    }

    #[must_use]
    pub const fn derived_type(&self) -> u8 {
        self.derived_type
    }

    #[must_use]
    pub const fn eui64(&self) -> &ember::Eui64 {
        &self.eui64
    }

    #[must_use]
    pub const fn multi_network_index(&self) -> u8 {
        self.multi_network_index
    }

    #[must_use]
    pub const fn flags(&self) -> u8 {
        self.flags
    }

    #[must_use]
    pub const fn psa_key_alg_permission(&self) -> u32 {
        self.psa_key_alg_permission
    }
}

#[cfg_attr(feature = "le-stream", derive(FromLeStream, ToLeStream))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManNetworkKeyInfo {
    network_key_set: bool,
    alternate_network_key_set: bool,
    network_key_sequence_number: u8,
    alt_network_key_sequence_number: u8,
    network_key_frame_counter: u32,
}

impl ManNetworkKeyInfo {
    #[must_use]
    pub const fn new(
        network_key_set: bool,
        alternate_network_key_set: bool,
        network_key_sequence_number: u8,
        alt_network_key_sequence_number: u8,
        network_key_frame_counter: u32,
    ) -> Self {
        Self {
            network_key_set,
            alternate_network_key_set,
            network_key_sequence_number,
            alt_network_key_sequence_number,
            network_key_frame_counter,
        }
    }

    #[must_use]
    pub const fn network_key_set(&self) -> bool {
        self.network_key_set
    }

    #[must_use]
    pub const fn alternate_network_key_set(&self) -> bool {
        self.alternate_network_key_set
    }

    #[must_use]
    pub const fn network_key_sequence_number(&self) -> u8 {
        self.network_key_sequence_number
    }

    #[must_use]
    pub const fn alt_network_key_sequence_number(&self) -> u8 {
        self.alt_network_key_sequence_number
    }

    #[must_use]
    pub const fn network_key_frame_counter(&self) -> u32 {
        self.network_key_frame_counter
    }
}

#[cfg_attr(feature = "le-stream", derive(FromLeStream, ToLeStream))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManApsKeyMetadata {
    bitmask: ember::KeyStructBitmask,
    outgoing_frame_counter: u32,
    incoming_frame_counter: u32,
    ttl_in_seconds: u16,
}

impl ManApsKeyMetadata {
    #[must_use]
    pub const fn new(
        bitmask: ember::KeyStructBitmask,
        outgoing_frame_counter: u32,
        incoming_frame_counter: u32,
        ttl_in_seconds: u16,
    ) -> Self {
        Self {
            bitmask,
            outgoing_frame_counter,
            incoming_frame_counter,
            ttl_in_seconds,
        }
    }

    #[must_use]
    pub const fn bitmask(&self) -> ember::KeyStructBitmask {
        self.bitmask
    }

    #[must_use]
    pub const fn outgoing_frame_counter(&self) -> u32 {
        self.outgoing_frame_counter
    }

    #[must_use]
    pub const fn incoming_frame_counter(&self) -> u32 {
        self.incoming_frame_counter
    }

    #[must_use]
    pub const fn ttl_in_seconds(&self) -> u16 {
        self.ttl_in_seconds
    }
}

/// Security Manager context flags.
///
/// # Documentation
/// [Link](https://docs.silabs.com/d/zigbee-stack-api/7.2.2/zigbee-security-manager#sl-zigbee-sec-man-flags-t).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum ManFlags {
    None = 0x00,
    KeyIndexIsValid = 0x01,
    EuiIsValid = 0x02,
    UnconfirmedTransientKey = 0x04,
}

impl From<ManFlags> for u8 {
    fn from(man_flags: ManFlags) -> Self {
        man_flags as Self
    }
}

impl TryFrom<u8> for ManFlags {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
