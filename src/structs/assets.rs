use std::collections::HashMap;

enum AssetType {
    Currency,
    Stock,
}
struct Asset {
    type_of: AssetType,
    name: String,
}

struct AssetInWallet {
    asset: Asset,
    quantity: f64,
}

struct Wallet {
    assets: HashMap<String, AssetInWallet>,
}

impl Wallet {
    fn add_asset(&mut self, assetInWallet: AssetInWallet) {
        self.assets
            .insert(assetInWallet.asset.name.clone(), assetInWallet);
    }
    fn new() -> Wallet {
        Wallet {
            assets: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_assets() {
        let mut wallet = Wallet::new();
        let asset1 = AssetInWallet {
            asset: Asset {
                type_of: AssetType::Stock,
                name: "Pineapple".to_string(),
            },
            quantity: 1000.0,
        };
        let asset2 = AssetInWallet {
            asset: Asset {
                type_of: AssetType::Stock,
                name: "Macrohard".to_string(),
            },

            quantity: 4000.0,
        };
        wallet.add_asset(asset1);
        wallet.add_asset(asset2);
        assert_eq!(wallet.assets["Pineapple"].quantity, 1000.0);
        assert_eq!(wallet.assets["Macrohard"].quantity, 4000.0);
        assert!(!wallet.assets.is_empty());
    }
}
