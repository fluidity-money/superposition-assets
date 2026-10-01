// SPDX-License-Identifier: Apache-2.0
pragma solidity >=0.8.0 <0.9.0;

// DEEPSEEK REIMPLEMENTATION OF THE CODE BELOW:

/// @notice The canonical asset registry, mirroring the `Asset` enum in
///         `superposition-assets/src/lib.rs`.
///
/// @dev BE MINDFUL: this enum must only ever be appended to with new items,
///      never have existing items reordered or deleted. The discriminants are
///      encoded on-chain (cast to/from `uint8`) and are stable for the lifetime
///      of the protocol. The Go reflection in `superposition_assets.go` is
///      stale and must not be treated as authoritative.
enum Asset {
    USDC, // 0
    ARB,  // 1
    WETH, // 2
    WBTC  // 3
}

/// @notice Network on which an asset token is deployed. Mirrors the `Network`
///         enum in `superposition-assets/src/lib.rs`. Only ever append.
enum Network {
    Arbitrum,
    Robinhood
}

/// @notice Conversion helpers between the internal `Asset` enum and its
///         on-chain `uint8` wire representation, mirroring the Rust impls in
///         `src/lib.rs` (`TryFrom<u8> for Asset`, `From<Asset> for u8`) and the
///         address lookups in `Asset::addr(self, n: Network)` / legacy
///         `Asset::addr(self)`.
library LibSuperpositionAssets {
    error InvalidAsset();
    error AssetNotOnNetwork(Asset asset, Network network);

    /// @notice Decodes a `uint8` wire value into an `Asset`.
    /// @dev    Reverts with `InvalidAsset` if `x` is not a known discriminant.
    ///         Equivalent to `TryFrom<u8> for Asset`. Appending a new asset to
    ///         `Asset` automatically widens the accepted range.
    function fromUint8(uint8 x) internal pure returns (Asset a) {
        if (x > uint8(Asset.WBTC)) revert InvalidAsset();
        return Asset(x);
    }

    /// @notice Encodes an `Asset` back into its `uint8` wire value.
    /// @dev    Equivalent to `From<Asset> for u8` (`x as u8`), i.e. it returns
    ///         the enum discriminant directly.
    function toUint8(Asset a) internal pure returns (uint8) {
        return uint8(a);
    }

    /// @notice Resolves the token address for `a` deployed on network `n`.
    /// @dev    Mirrors `Asset::addr(self, n: Network) -> [u8; 20]`. ARB is not
    ///         deployed on Robinhood; the Rust code panics there, here we revert
    ///         with `AssetNotOnNetwork`.
    function addr(Asset a, Network n) internal pure returns (address) {
        if (n == Network.Arbitrum) {
            if (a == Asset.USDC) return 0xaf88d065e77c8cC2239327C5EDb3A432268e5831;
            if (a == Asset.ARB) return 0x912CE59144191C1204E64559FE8253a0e49E6548;
            if (a == Asset.WETH) return 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1;
            if (a == Asset.WBTC) return 0x2f2a2543B76A4166549F7aaB2e75Bef0aefC5B0f;
        } else {
            // n == Network.Robinhood
            if (a == Asset.USDC) return 0x80e0e24718dbFcad49ECAA6F1e6C89A190586cA8;
            if (a == Asset.ARB) revert AssetNotOnNetwork(a, n);
            if (a == Asset.WETH) return 0x0Bd7D308f8E1639FAb988df18A8011f41EAcAD73;
            if (a == Asset.WBTC) return 0x6bac06600D220Ac5Ac281AD1f504D2Cf0F90F6e6;
        }
        revert InvalidAsset();
    }

    /// @notice Returns the token address associated with `a` — the canonical,
    ///         network-independent address this protocol associates each asset
    ///         with. Mirrors the legacy `Asset::addr(self) -> [u8; 20]` used by
    ///         the accounts lib (`asset.addr()`), i.e. the Arbitrum One set.
    /// @dev    Equivalent to `addr(a, Network.Arbitrum)`.
    function associatedAddr(Asset a) internal pure returns (address) {
        if (a == Asset.USDC) return 0xaf88d065e77c8cC2239327C5EDb3A432268e5831;
        if (a == Asset.ARB) return 0x912CE59144191C1204E64559FE8253a0e49E6548;
        if (a == Asset.WETH) return 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1;
        if (a == Asset.WBTC) return 0x2f2a2543B76A4166549F7aaB2e75Bef0aefC5B0f;
        revert InvalidAsset();
    }
}
