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

/// @notice Conversion helpers between the internal `Asset` enum and its
///         on-chain `uint8` wire representation. Mirrors the Rust impls
///         `TryFrom<u8> for Asset` and `From<Asset> for u8` in `src/lib.rs`.
library LibSuperpositionAssets {
    error InvalidAsset();

    /// @notice Decodes a `uint8` wire value into an `Asset`.
    /// @dev    Reverts with `InvalidAsset` if `x` is not a known discriminant.
    ///         Equivalent to `TryFrom<u8> for Asset` (the `Err(_) => InvalidAsset`
    ///         branch). Appending a new asset to `Asset` automatically widens the
    ///         accepted range.
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
}
