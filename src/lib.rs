#![no_std]

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asset {
    USDC = 0,
    ARB = 1,
    WETH = 2
}

const fn decode(x: &[u8]) -> [u8; 20] {
    match const_hex::const_decode_to_array::<20>(x) {
        Ok(r) => r,
        Err(_) => panic!(),
    }
}

impl From<Asset> for [u8; 20] {
    fn from(x: Asset) -> Self {
        match x {
            Asset::USDC => decode(b"af88d065e77c8cC2239327C5EDb3A432268e5831"),
            Asset::ARB => decode(b"912ce59144191c1204e64559fe8253a0e49e6548"),
            Asset::WETH => decode(b"82af49447d8a07e3bd95bd0d56f35241523fbab1"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidAsset;

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
