use std::collections::HashMap;

pub type Qty = f64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssetType {
    Currency,
    Stock,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Asset {
    pub type_of: AssetType,
    pub name: String,
}

impl Asset {
    pub fn new(type_of: AssetType, name: String) -> Self {
        Self { type_of, name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

struct AssetInWallet {
    asset: Asset,
    quantity: Qty,
}

impl AssetInWallet {
    fn asset(&self) -> &Asset {
        &self.asset
    }
}

struct Wallet {
    assets: HashMap<String, AssetInWallet>,
}

impl Wallet {
    fn add_asset(&mut self, asset_in_wallet: AssetInWallet) {
        self.assets
            .insert(asset_in_wallet.asset.name.clone(), asset_in_wallet);
    }
    fn new() -> Wallet {
        Wallet {
            assets: HashMap::new(),
        }
    }

    fn assets_mut(&mut self) -> &mut HashMap<String, AssetInWallet> {
        &mut self.assets
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
