package superposition_assets

import (
	"fmt"
	"strings"
)

type Asset uint8

const (
	AssetUsdc Asset = iota
	AssetArb
	AssetWeth
	AssetWbtc
	AssetUsdg
)

type Network uint8

const (
	NetworkArbitrum Network = iota
	NetworkRobinhood
)

func (a Asset) String() string {
	switch a {
	case AssetUsdc:
		return "USDC"
	case AssetArb:
		return "ARB"
	case AssetWeth:
		return "WETH"
	case AssetWbtc:
		return "WBTC"
	case AssetUsdg:
		return "USDG"
	}
	return ""
}

func AssetFromString(x string) (Asset, error) {
	switch strings.ToUpper(x) {
	case "USDC":
		return AssetUsdc, nil
	case "ARB":
		return AssetArb, nil
	case "WETH":
		return AssetWeth, nil
	case "WBTC":
		return AssetWbtc, nil
	case "USDG":
		return AssetUsdg, nil
	default:
		return 0, fmt.Errorf("unknown asset %q", x)
	}
}

// Address resolves the token address for the asset on the given network.
func (a Asset) Address(n Network) (string, bool) {
	switch n {
	case NetworkArbitrum:
		switch a {
		case AssetUsdc:
			return "0xaf88d065e77c8cc2239327c5edb3a432268e5831", true
		case AssetArb:
			return "0x912ce59144191c1204e64559fe8253a0e49e6548", true
		case AssetWeth:
			return "0x82af49447d8a07e3bd95bd0d56f35241523fbab1", true
		case AssetWbtc:
			return "0x2f2a2543b76a4166549f7aab2e75bef0aefc5b0f", true
		case AssetUsdg:
			return "0x004b506865409877c9fa29bfb1eba929984b9bbc", true
		}
	case NetworkRobinhood:
		switch a {
		case AssetWeth:
			return "0x0bd7d308f8e1639fab988df18a8011f41eacad73", true
		case AssetWbtc:
			return "0x6bac06600d220ac5ac281ad1f504d2cf0f90f6e6", true
		case AssetUsdg:
			return "0x5fc5360d0400a0fd4f2af552add042d716f1d168", true
		}
	}
	return "", false
}
