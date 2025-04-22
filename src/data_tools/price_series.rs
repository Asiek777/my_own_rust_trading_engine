use crate::structs::{
    assets::{Asset, Qty},
    quotes::Price,
};
use chrono::NaiveDate;

#[derive(Debug)]
pub struct PriceEntry {
    pub open: Price,
    pub high: Price,
    pub low: Price,
    pub close: Price,
    pub volume: Qty,
    pub date: NaiveDate,
}

impl PriceEntry {
    fn new(
        open: Price,
        high: Price,
        low: Price,
        close: Price,
        volume: Qty,
        date: NaiveDate,
    ) -> Self {
        Self {
            open,
            high,
            low,
            close,
            volume,
            date,
        }
    }
}

#[derive(Debug)]
pub struct PriceSeries {
    asset: Asset,
    rate: String,
    entries: Vec<PriceEntry>,
}

impl PriceSeries {
    fn new(asset: Asset, rate: String, entries: Vec<PriceEntry>) -> Self {
        Self {
            asset,
            rate,
            entries,
        }
    }

    pub fn entries(&self) -> &[PriceEntry] {
        &self.entries
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::fs;

    pub fn to_price(slice: &str) -> Price {
        let number = slice.parse::<f64>().unwrap() * 10000.0;
        number as Price
    }
    pub fn line_to_entry(line: &str) -> PriceEntry {
        let mut entry = line.split(",").skip(2);
        let date_string = entry.next().unwrap();
        let date = NaiveDate::parse_from_str(date_string, "%Y%m%d").unwrap();
        entry.next();
        let open = to_price(entry.next().unwrap());
        let high = to_price(entry.next().unwrap());
        let low = to_price(entry.next().unwrap());
        let close = to_price(entry.next().unwrap());
        let volume = entry.next().unwrap().parse::<Qty>().unwrap();
        PriceEntry::new(open, high, low, close, volume, date)
    }
    pub fn file_to_price_serises(content: String) -> PriceSeries {
        let lines = content.lines();
        let mut series = PriceSeries::new(
            Asset::new(
                crate::structs::assets::AssetType::Currency,
                "ORL".to_owned(),
            ),
            "daily".to_owned(),
            Vec::new(),
        );
        for line in lines.skip(1) {
            let entry = line_to_entry(line);
            series.entries.push(entry);
        }
        series
    }

    #[test]
    fn reading_entries() {
        let content = fs::read_to_string("./training_data/nc stocks/orl.txt").unwrap();
        let series = file_to_price_serises(content);
        print!("{:?}", series);
        assert_eq!(2609, series.entries.len())
    }
}
