use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};

/// AS number length: 16 or 32 bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AsnLength {
    Bits16,
    Bits32,
}

/// ASN -- Autonomous System Number
#[derive(Debug, Clone, Copy, Eq)]
pub struct Asn {
    pub asn: u32,
    pub len: AsnLength,
}

impl PartialEq for Asn {
    fn eq(&self, other: &Self) -> bool {
        self.asn == other.asn
    }
}

impl PartialEq<i32> for Asn {
    fn eq(&self, other: &i32) -> bool {
        self.asn as i32 == *other
    }
}

impl PartialEq<u32> for Asn {
    fn eq(&self, other: &u32) -> bool {
        self.asn == *other
    }
}

impl From<u32> for Asn {
    fn from(v: u32) -> Self {
        Asn {
            asn: v,
            len: AsnLength::Bits32,
        }
    }
}

impl From<i32> for Asn {
    fn from(v: i32) -> Self {
        Asn {
            asn: v as u32,
            len: AsnLength::Bits32,
        }
    }
}

impl From<Asn> for i32 {
    fn from(val: Asn) -> Self {
        val.asn as i32
    }
}

impl From<Asn> for u32 {
    fn from(value: Asn) -> Self {
        value.asn
    }
}

impl Serialize for Asn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.asn)
    }
}

impl<'de> Deserialize<'de> for Asn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        u32::deserialize(deserializer).map(Asn::from)
    }
}

impl Display for Asn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.asn)
    }
}
