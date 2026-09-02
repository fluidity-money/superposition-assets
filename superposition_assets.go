package superposition_assets

type Asset uint8

const (
	AssetUsdc Asset = iota
	AssetArb
	AssetWeth
)

func AssetFromString(x string) (Asset, error) {
	switch strings.ToUpper(x) {
	case "USDC":
		return AssetUsdc, nil
	case "ARB":
		return AssetArb, nil
	case "WETH":
		return AssetWeth, nil
	default:
		return 0, fmt.Errorf("unknown asset %q", x)
	}
}

func AssetToAddress(a Asset) string {
	match a {
	case AssetUsdc:
		return "0xaf88d065e77c8cC2239327C5EDb3A432268e5831"
	case AssetArb:
		return "0x912ce59144191c1204e64559fe8253a0e49e6548"
	case AssetWeth:
		return "0x82af49447d8a07e3bd95bd0d56f35241523fbab1";
	}
}
