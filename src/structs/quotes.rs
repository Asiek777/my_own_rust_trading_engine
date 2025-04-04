use super::assets::{Asset, Qty};

pub type Price = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
pub struct Quote {
    asset: Asset,
    side: Side,
    quantity: Qty,
    price: Price, //    currency: String,
}

impl Quote {
    pub fn new(asset: Asset, side: Side, quantity: Qty, price: Price) -> Self {
        Self {
            asset,
            side,
            quantity,
            price,
        }
    }

    pub fn asset(&self) -> &Asset {
        &self.asset
    }

    pub fn side(&self) -> &Side {
        &self.side
    }

    pub fn quantity(&self) -> &Qty {
        &self.quantity
    }

    pub fn price(&self) -> &Price {
        &self.price
    }
}
