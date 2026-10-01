#![cfg_attr(not(any(feature = "proptest", feature = "arbitrary")), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::String;

mod version;

pub use version::Version;

pub const VERSION_CURRENT: Version = Version::CURRENT;

use bobcat_maths::U;

use bobcat_cd::{EvmCdDeserialise, EvmCdSerialise};

use stylus_robinhood_stock_tokens::StockToken::Spy as RobinhoodSpy;

use core::str::FromStr;

#[repr(u8)]
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshDeserialize, borsh::BorshSerialize),
    borsh(use_discriminant = true)
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(
    feature = "evm-cd",
    derive(EvmCdSerialise, EvmCdDeserialise),
    evm_values
)]
pub enum Asset {
    USDC = 0,
    ARB = 1,
    WETH = 2,
    WBTC = 3,
    USDG = 4,
    SPY = 5,
}

#[repr(u8)]
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshDeserialize, borsh::BorshSerialize),
    borsh(use_discriminant = true)
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(
    feature = "evm-cd",
    derive(EvmCdSerialise, EvmCdDeserialise),
    evm_values
)]
pub enum Network {
    Arbitrum,
    Robinhood,
    RobinhoodTestnet,
}

impl Asset {
    pub fn addr(self, n: Network) -> Option<[u8; 20]> {
        const fn decode(x: &[u8]) -> [u8; 20] {
            match const_hex::const_decode_to_array::<20>(x) {
                Ok(r) => r,
                Err(_) => panic!(),
            }
        }
        match (self, n) {
            (Asset::USDC, Network::Arbitrum) => {
                Some(decode(b"af88d065e77c8cC2239327C5EDb3A432268e5831"))
            }
            (Asset::ARB, Network::Arbitrum) => {
                Some(decode(b"912ce59144191c1204e64559fe8253a0e49e6548"))
            }
            (Asset::WETH, Network::Arbitrum) => {
                Some(decode(b"82af49447d8a07e3bd95bd0d56f35241523fbab1"))
            }
            (Asset::WBTC, Network::Arbitrum) => {
                Some(decode(b"2f2a2543b76a4166549f7aab2e75bef0aefc5b0f"))
            }
            (Asset::USDG, Network::Arbitrum) => {
                Some(decode(b"004b506865409877c9fa29bfb1eba929984b9bbc"))
            }
            (Asset::USDC, Network::Robinhood) => None,
            (Asset::WETH, Network::Robinhood) => {
                Some(decode(b"0Bd7D308f8E1639FAb988df18A8011f41EAcAD73"))
            }
            (Asset::WBTC, Network::Robinhood) => {
                Some(decode(b"6bac06600D220Ac5Ac281AD1f504D2Cf0F90F6e6"))
            }
            (Asset::USDG, Network::Robinhood) => {
                Some(decode(b"5fc5360d0400a0fd4f2af552add042d716f1d168"))
            }
            (Asset::ARB, Network::Robinhood) => None,
            (Asset::SPY, Network::Arbitrum) => None,
            (Asset::SPY, Network::Robinhood) => Some(RobinhoodSpy.addr()),
            (Asset::USDC, Network::RobinhoodTestnet) => None,
            (Asset::ARB, Network::RobinhoodTestnet) => None,
            (Asset::WETH, Network::RobinhoodTestnet) => None,
            (Asset::WBTC, Network::RobinhoodTestnet) => None,
            (Asset::USDG, Network::RobinhoodTestnet) => None,
            // This is not really the SPY token! This is a fake asset deployed by the Superposition team for Florin.
            (Asset::SPY, Network::RobinhoodTestnet) => {
                Some(decode(b"2541F59c5e47cC36eE1368d0F8B96360791D708f"))
            }
        }
    }

    pub fn u(self, n: Network) -> Option<U> {
        self.addr(n).map(U::from)
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

impl core::fmt::Display for InvalidAsset {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl core::error::Error for InvalidAsset {}

impl FromStr for Asset {
    type Err = InvalidAsset;

    fn from_str(x: &str) -> Result<Self, Self::Err> {
        match x {
            "usdc" | "USDC" => Ok(Asset::USDC),
            "arb" | "ARB" => Ok(Asset::ARB),
            "weth" | "WETH" => Ok(Asset::WETH),
            "wbtc" | "WBTC" => Ok(Asset::WBTC),
            "usdg" | "USDG" => Ok(Asset::USDG),
            "spy" | "SPY" => Ok(Asset::SPY),
            _ => Err(InvalidAsset),
        }
    }
}

impl Asset {
    pub fn from_str(x: &str) -> Self {
        x.into()
    }

    pub fn try_from_str(x: &str) -> Result<Self, InvalidAsset> {
        x.try_into().map_err(|_| InvalidAsset)
    }

    #[cfg(feature = "alloc")]
    pub fn from_string(x: String) -> Self {
        x.into()
    }

    #[cfg(feature = "alloc")]
    pub fn try_from_string(x: String) -> Result<Self, InvalidAsset> {
        x.try_into().map_err(|_| InvalidAsset)
    }

    pub const fn const_id(self) -> [u8; 4] {
        let mut b = [0u8; 4];
        b.copy_from_slice(
            match self {
                Asset::USDC => "USDC",
                Asset::ARB => "ARB_",
                Asset::WETH => "WETH",
                Asset::WBTC => "WBTC",
                Asset::USDG => "USDG",
                Asset::SPY => "SPY_",
            }
            .as_bytes(),
        );
        b
    }
}

impl Into<&str> for Asset {
    fn into(self) -> &'static str {
        match self {
            Asset::USDC => "USDC",
            Asset::ARB => "ARB",
            Asset::WETH => "WETH",
            Asset::WBTC => "WBTC",
            Asset::USDG => "USDG",
            Asset::SPY => "SPY",
        }
    }
}
impl From<&str> for Asset {
    fn from(x: &str) -> Self {
        x.parse().unwrap_or_else(|_| panic!("bad asset: {x}"))
    }
}

impl TryFrom<u8> for Asset {
    type Error = InvalidAsset;

    fn try_from(x: u8) -> Result<Self, Self::Error> {
        match x {
            0 => Ok(Asset::USDC),
            1 => Ok(Asset::ARB),
            2 => Ok(Asset::WETH),
            3 => Ok(Asset::WBTC),
            4 => Ok(Asset::USDG),
            5 => Ok(Asset::SPY),
            _ => Err(InvalidAsset),
        }
    }
}

impl TryFrom<U> for Asset {
    type Error = InvalidAsset;

    fn try_from(x: U) -> Result<Self, Self::Error> {
        Self::try_from(x[31])
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
