// SPDX-License-Identifier: Apache-2.0
pragma solidity >=0.8.0 <0.9.0;

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

/// @notice Conversion helpers for the internal `Asset` enum: uint8 wire
///         encoding/decoding and per-network token address lookups. Mirrors the
///         Rust impls `TryFrom<u8> for Asset`, `From<Asset> for u8`, and
///         `Asset::addr(self, n: Network)` in `src/lib.rs`.
///
/// @dev Address lookups are split into explicit per-network functions
///      (`addrArbitrum`, `addrRobinhood`) rather than a single network-switching
///      routine so the generated code is a flat, branch-free per-asset dispatch
///      and never relies on the compiler inlining an internal switch.
library LibSuperpositionAssets {
    error InvalidAsset();
    error AssetNotOnNetwork(Asset asset);

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

    /// @notice Resolves `a` to its token address on Arbitrum One.
    /// @dev    Mirrors `Asset::addr(self, Network::Arbitrum)`.
    function addrArbitrum(Asset a) internal pure returns (address) {
        if (a == Asset.USDC) return 0xaf88d065e77c8cC2239327C5EDb3A432268e5831;
        if (a == Asset.ARB) return 0x912CE59144191C1204E64559FE8253a0e49E6548;
        if (a == Asset.WETH) return 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1;
        if (a == Asset.WBTC) return 0x2f2a2543B76A4166549F7aaB2e75Bef0aefC5B0f;
        revert InvalidAsset();
    }

    /// @notice Resolves `a` to its token address on the Robinhood chain.
    /// @dev    Mirrors `Asset::addr(self, Network::Robinhood)`. USDC and ARB are
    ///         not deployed on Robinhood; the Rust code panics there, here we
    ///         revert with `AssetNotOnNetwork`. Only WETH and WBTC exist.
    function addrRobinhood(Asset a) internal pure returns (address) {
        if (a == Asset.USDC) revert AssetNotOnNetwork(a);
        if (a == Asset.ARB) revert AssetNotOnNetwork(a);
        if (a == Asset.WETH) return 0x0Bd7D308f8E1639FAb988df18A8011f41EAcAD73;
        if (a == Asset.WBTC) return 0x6bac06600D220Ac5Ac281AD1f504D2Cf0F90F6e6;
        revert InvalidAsset();
    }
}