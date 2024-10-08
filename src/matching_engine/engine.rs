use std::collections::HashMap;
use colored::Colorize;
use rust_decimal::Decimal;
use uuid::Uuid;

use super::orderbook::{Order, Orderbook};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair { base, quote }
    }

    pub fn to_string(&self) -> String {
        format!("{}-{}", self.base, self.quote)
    }
}

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, Orderbook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair: &TradingPair) {
        self.orderbooks.insert(pair.clone(), Orderbook::new());
        println!("opening new orderbook for market {:?}", pair.to_string());
    }

    pub fn place_limit_order(&mut self, pair: &TradingPair, price: Decimal, order: Order)
     -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_limit_order(price, order);

                println!("placed limit order at price level {}", price);
                Ok(())
            },
            None => {
                Err(format!(
                    "The orderbook for the given trading pair ({}) doesn't exist!",
                    pair.to_string()))
            }
        }
    }

    pub fn place_market_order(&mut self, pair: &TradingPair, order: &mut Order)
    -> Result<(), String>  {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.fill_market_order(order);

                println!("{}", "placed market order!".green());
                Ok(())
            },
            None => {
                Err(format!(
                    "The orderbook for the given trading pair ({}) doesn't exist!",
                    pair.to_string()))
            }
        }
    }

    pub fn display_orderbook(&mut self, pair: &TradingPair) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.display();
                Ok(())
            },
            None => {
                Err(format!(
                    "The orderbook for the given trading pair ({}) doesn't exist!",
                    pair.to_string()))
            }
        }
    }

    pub fn cancel_limited_order(&mut self, pair: &TradingPair, id: Uuid) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.cancel_limited_order(id);

                println!("The order was cancelled");
                Ok(())
            },
            None => {
                Err(format!(
                    "The orderbook for the given trading pair ({}) doesn't exist!",
                    pair.to_string()))
            }
        }
    }
}
