use super::{
    assets::{Asset, Qty},
    quotes::{Price, Quote, Side},
};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
struct OrderBookEntry {
    side: Side,
    quantity: Qty,
}

impl OrderBookEntry {
    fn new(side: Side, quantity: Qty) -> Self {
        Self { side, quantity }
    }
    fn sum_entries(&mut self, entry: OrderBookEntry) {
        if self.side == entry.side {
            self.quantity += entry.quantity;
        } else if self.quantity > entry.quantity {
            self.quantity -= entry.quantity;
        } else {
            self.side = entry.side;
            self.quantity = entry.quantity - self.quantity;
        }
    }
}

struct OrderBook {
    asset: Asset,
    currency: String,
    book: BTreeMap<Price, OrderBookEntry>,
}

impl OrderBook {
    fn new(asset: Asset, currency: String, book: BTreeMap<Price, OrderBookEntry>) -> Self {
        Self {
            asset,
            currency,
            book,
        }
    }
    fn add_quote(&mut self, quote: Quote) {
        debug_assert_eq!(self.asset, *quote.asset());
        let new_entry = OrderBookEntry::new(quote.side().reverse(), *quote.quantity());
        self.add_order(*quote.price(), new_entry);
    }

    fn add_order(&mut self, price: Price, new_entry: OrderBookEntry) {
        let old_entry = self.book.get_mut(&price);
        if let Some(entry) = old_entry {
            entry.sum_entries(new_entry);
        } else {
            self.book.insert(price, new_entry);
        }
    }

    fn get_entry(&self, price: &Price) -> Option<&OrderBookEntry> {
        self.book.get(price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::assets::AssetType;
    #[test]
    fn inserting_assets() {
        let asset = Asset::new(AssetType::Stock, "Pineapple".to_string());
        let currency = "USD".into();
        let mut book = OrderBook::new(asset.clone(), currency, BTreeMap::new());

        let price1 = 10;
        let qty1 = 1000.0;
        let quote1 = Quote::new(asset.clone(), Side::Sell, qty1, price1);
        book.add_quote(quote1);

        assert_eq!(
            *book.get_entry(&price1).unwrap(),
            OrderBookEntry::new(Side::Buy, qty1)
        );

        let qty2 = 25.2;
        let quote2 = Quote::new(asset.clone(), Side::Sell, qty2, price1);
        book.add_quote(quote2);
        assert_eq!(
            *book.get_entry(&price1).unwrap(),
            OrderBookEntry::new(Side::Buy, qty1 + qty2)
        );

        let quote3 = Quote::new(asset.clone(), Side::Buy, 2000.0, price1);
        book.add_quote(quote3);
        assert_eq!(
            *book.get_entry(&price1).unwrap(),
            OrderBookEntry::new(Side::Sell, 2000.0 - qty1 - qty2)
        );
    }
}
