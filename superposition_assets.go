package superposition_assets

import (
	"strings"
	"fmt"
)

type Asset uint8

const (
	AssetUsdc Asset = iota
	AssetArb
	AssetWeth
	AssetUsdg
)

func AssetFromString(x string) (Asset, error) {
	switch strings.ToUpper(x) {
	case "USDC":
		return AssetUsdc, nil
	case "ARB":
		return AssetArb, nil
	case "WETH":
		return AssetWeth, nil
	case "USDG":
		return AssetUsdg, nil
	default:
		return 0, fmt.Errorf("unknown asset %q", x)
	}
}

func (a Asset) Address() string {
	switch a {
	case AssetUsdc:
		return "0xaf88d065e77c8cc2239327c5edb3a432268e5831"
	case AssetArb:
		return "0x912ce59144191c1204e64559fe8253a0e49e6548"
	case AssetWeth:
		return "0x82af49447d8a07e3bd95bd0d56f35241523fbab1";
	}
	return ""
}
