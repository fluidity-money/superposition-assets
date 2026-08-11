#![cfg_attr(not(any(feature = "proptest", feature = "arbitrary")), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::String;

use core::str::FromStr;

#[repr(u8)]
#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshDeserialize, borsh::BorshSerialize),
    borsh(use_discriminant = true)
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum Asset {
    USDC = 0,
    ARB = 1,
    WETH = 2,
}

const fn decode(x: &[u8]) -> [u8; 20] {
    match const_hex::const_decode_to_array::<20>(x) {
        Ok(r) => r,
        Err(_) => panic!(),
    }
}

impl Asset {
    fn addr(&self) -> [u8; 20] {
        match self {
            Asset::USDC => decode(b"af88d065e77c8cC2239327C5EDb3A432268e5831"),
            Asset::ARB => decode(b"912ce59144191c1204e64559fe8253a0e49e6548"),
            Asset::WETH => decode(b"82af49447d8a07e3bd95bd0d56f35241523fbab1"),
        }
    }
}

impl From<Asset> for [u8; 20] {
    fn from(x: Asset) -> Self {
        x.addr()
    }
}

impl From<&Asset> for [u8; 20] {
    fn from(x: &Asset) -> Self {
        x.addr()
    }
}

#[cfg(feature = "alloc")]
impl From<String> for Asset {
    fn from(x: String) -> Self {
        x.as_str().into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidAsset;

impl FromStr for Asset {
    type Err = InvalidAsset;

    fn from_str(x: &str) -> Result<Self, Self::Err> {
        match x {
            "usdc" | "USDC" => Ok(Asset::USDC),
            "arb" | "ARB" => Ok(Asset::ARB),
            "weth" | "WETH" => Ok(Asset::WETH),
            _ => Err(InvalidAsset),
        }
    }
}

impl Asset {
    pub fn from_str(x: &str) -> Self {
        x.into()
    }
}

impl From<&str> for Asset {
    fn from(x: &str) -> Self {
        x.parse()
            .unwrap_or_else(|_| panic!("bad asset: {x}"))
    }
}

impl TryFrom<u8> for Asset {
    type Error = InvalidAsset;

    fn try_from(x: u8) -> Result<Self, Self::Error> {
        match x {
            0 => Ok(Asset::USDC),
            1 => Ok(Asset::ARB),
            2 => Ok(Asset::WETH),
            _ => Err(InvalidAsset),
        }
    }
}

macro_rules! impl_asset_int {
    ($($ty:ty),* $(,)?) => {
        $(
            impl TryFrom<$ty> for Asset {
                type Error = InvalidAsset;

                fn try_from(x: $ty) -> Result<Self, Self::Error> {
                    let x = u8::try_from(x).map_err(|_| InvalidAsset)?;
                    Asset::try_from(x)
                }
            }

            impl From<Asset> for $ty {
                fn from(x: Asset) -> Self {
                    x as u8 as $ty
                }
            }
        )*
    };
}

impl_asset_int!(u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
