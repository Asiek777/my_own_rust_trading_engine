use chrono::NaiveDate;

use crate::structs::quotes::Price;

use super::price_series::{self, PriceSeries};

#[derive(Debug)]
enum IndicatorType {
    RSI(u16),
}

impl From<u16> for IndicatorType {
    fn from(v: u16) -> Self {
        Self::RSI(v)
    }
}

#[derive(Debug)]
struct IndicatorValue {
    value: f32,
    date: NaiveDate,
}

impl IndicatorValue {
    fn new(value: f32, date: NaiveDate) -> Self {
        Self { value, date }
    }
}

#[derive(Debug)]
struct IndicatorSeries<'a> {
    typ: IndicatorType,
    price_series: &'a PriceSeries,
    values: Vec<IndicatorValue>,
}

impl<'a> IndicatorSeries<'a> {
    fn new(typ: IndicatorType, price_series: &'a PriceSeries) -> Self {
        let values = Self::calc_indicator(&typ, price_series);
        Self {
            typ,
            price_series,
            values,
        }
    }

    fn calc_indicator(typ: &IndicatorType, price_series: &PriceSeries) -> Vec<IndicatorValue> {
        return match typ {
            IndicatorType::RSI(window) => Self::calc_rsi(price_series, *window),
        };
    }

    fn calc_rsi(price_series: &PriceSeries, time_window: u16) -> Vec<IndicatorValue> {
        let window = time_window as u32;
        let mut close_changes = vec![0; time_window as usize];
        let mut close_number: u32 = 0;
        let mut result = Vec::new();

        fn calc_single_rsi(close_changes: &Vec<i64>) -> f32 {
            let mut pos = 0;
            let mut pos_count = 0;
            let mut negs = 0;
            let mut neg_count = 0;
            for val in close_changes {
                if *val > 0 {
                    pos += val;
                    pos_count += 1;
                } else {
                    negs -= val;
                    neg_count += 1;
                }
            }
            if neg_count == 0 {
                return 100.0;
            }
            let pos_av: f32 = pos as f32 / pos_count as f32;
            let neg_av: f32 = negs as f32 / neg_count as f32;
            100.0 - (100.0 / (1.0 + pos_av / neg_av))
        }

        let mut last_close = price_series.entries()[0].close;
        for entry in price_series.entries().iter().skip(1) {
            close_changes[(close_number % window) as usize] =
                entry.close as i64 - last_close as i64;
            last_close = entry.close;
            if close_number > window {
                result.push(IndicatorValue::new(
                    calc_single_rsi(&close_changes),
                    entry.date,
                ));
                // println!("{:?}", result.last().unwrap());
            }
            close_number += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    //This test is not actually a test
    #[test]
    fn rsi_calc() {
        let content = fs::read_to_string("./training_data/nc stocks/orl.txt").unwrap();
        let series = price_series::tests::file_to_price_serises(content);
        let indicator_series = IndicatorSeries::new(IndicatorType::RSI(14), &series);
        for val in indicator_series.values {
            println!("{}\t{}", val.value, val.date);
        }
        assert!(true);
    }
}
