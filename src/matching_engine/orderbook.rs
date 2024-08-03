#![allow(dead_code)]
use std::collections::HashMap;
use rust_decimal::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum BidOrAsk {
    Bid, 
    Ask,
}

#[derive(Debug)]
pub struct Orderbook {
    asks: HashMap<Decimal, Limit>,
    bids: HashMap<Decimal, Limit>,
    order_map: HashMap<Uuid, (Decimal, BidOrAsk)>
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new(),
            order_map: HashMap::new()
        }
    }

    pub fn display(&mut self) {
        println!("The state of the Orderbook");
        println!("==========================");
        println!("ASK");
        for order in self.ask_limits().iter().rev() {
            println!("{}: {}", order.price, order.total_volume());
        }
        println!("--------------------------");
        for order in self.bid_limits().iter() {
            println!("{}: {}", order.price, order.total_volume());
        }
        println!("BID");
        println!("==========================");
    }

    pub fn cancel_limited_order(&mut self, id: Uuid) {
        if let Some((price, side)) = self.order_map.get(&id) {
            match side {
                BidOrAsk::Bid => {
                    match self.bids.get_mut(&price) {
                        Some(limit) => {
                            limit.cancel_order(id);
                            if limit.total_volume() == 0.0 {
                                self.bids.remove(price);
                            }
                        },
                        None => println!("The order doesn't exist, id: {}", id)
                    }
                },
                BidOrAsk::Ask => {
                    match self.asks.get_mut(&price) {
                        Some(limit) => {
                            limit.cancel_order(id);
                            if limit.total_volume() == 0.0 {
                                self.asks.remove(price);
                            }
                        },
                        None => println!("The order doesn't exist, id: {}", id)
                    } 
                }
            }

            self.order_map.remove(&id);
        } else {
            println!("The order doesn't exist");
        }
    }

    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        let limits = match market_order.bid_or_ask {
            BidOrAsk::Bid => self.ask_limits(),
            BidOrAsk::Ask => self.bid_limits()
        };

        for limit_order in limits {
            limit_order.fill_order(market_order);

            if market_order.is_filled() {
                break;
            }
        }
    }

    pub fn ask_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.asks.values_mut().collect::<Vec<&mut Limit>>();
        
        limits.sort_by(|a, b| a.price.cmp(&b.price));

        limits
    }

    pub fn bid_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.bids.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| b.price.cmp(&a.price));

        limits
    }

    pub fn add_limit_order(&mut self, price: Decimal, order: Order) {
        let order_id = order.id;
        let bid_or_ask = order.bid_or_ask.clone();

        match order.bid_or_ask {
            BidOrAsk::Bid => {
                let limit = self.bids.get_mut(&price);
                match limit {
                    Some(limit) => limit.add_order(order),
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            },
            BidOrAsk::Ask => {
                let limit = self.asks.get_mut(&price);
                match limit {
                    Some(limit) => limit.add_order(order),
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.asks.insert(price, limit);
                    }
                }
            },
        }

        self.order_map.insert(order_id, (price, bid_or_ask));
    }
}

#[derive(Debug)]
pub struct Limit {
    price: Decimal,
    orders: Vec<Order>
}

impl Limit {
   pub fn new(price: Decimal) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    fn total_volume(&self) -> f64 {
        self.orders
        .iter()
        .map(|order| order.size)
        .sum() 
    }

    fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0
                },
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0
                }
           }

           if market_order.is_filled() {
            break;
           }
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }

    pub fn cancel_order(&mut self, id: Uuid) {
        let mut i = 0;
        while i < self.orders.len() {
            if self.orders[i].id == id {
                self.orders.remove(i);
                break;
            }
            i += 1;
        }
    }

}

#[derive(Debug, Clone)]
pub struct Order {
    id: Uuid,
    size: f64,
    bid_or_ask: BidOrAsk
}

impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { bid_or_ask , size, id: Uuid::new_v4() }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    } 
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    
    #[test]
    fn cancel_order() {
        let mut orderbook = Orderbook::new();
        let buy_order_a = Order::new(BidOrAsk::Bid, 10.0);
        let buy_order_b = Order::new(BidOrAsk::Bid, 10.0);
        let cancelled_order = buy_order_a.clone();

        let sell_order = Order::new(BidOrAsk::Ask, 10.0);

        orderbook.add_limit_order(dec!(99.0), buy_order_a);
        orderbook.add_limit_order(dec!(99.0), buy_order_b);
        orderbook.add_limit_order(dec!(101.0), sell_order);
        orderbook.display();

        orderbook.cancel_limited_order(cancelled_order.id);
        orderbook.display();

        assert_eq!(orderbook.bids.get(&dec!(99.0)).unwrap().total_volume(), 10.0); 
    }

    #[test]
    fn orderbook_fill_market_order_ask() {
        let mut orderbook = Orderbook::new();
        orderbook.add_limit_order(dec!(500), Order::new(BidOrAsk::Ask, 10.0));
        orderbook.add_limit_order(dec!(100), Order::new(BidOrAsk::Ask, 10.0));
        orderbook.add_limit_order(dec!(200), Order::new(BidOrAsk::Ask, 10.0));
        orderbook.add_limit_order(dec!(300), Order::new(BidOrAsk::Ask, 10.0));
        
        let mut market_order = Order::new(BidOrAsk::Bid, 10.0);
        orderbook.fill_market_order(&mut market_order);

        let ask_limits = orderbook.ask_limits();
        let matched_limit = ask_limits.get(0).unwrap(); 
        
        assert_eq!(matched_limit.price, dec!(100));
        assert_eq!(market_order.is_filled(), true);

        let matched_order = matched_limit.orders.get(0).unwrap();
        assert_eq!(matched_order.is_filled(), true);

        println!("{:?}", orderbook.ask_limits());
    }

    #[test]
    fn limit_total_volume() {
        let price = dec!(10000.0);
        let mut limit = Limit::new(price);
        let buy_limit_order_a = Order::new(BidOrAsk::Bid, 100.0);
        let buy_limit_order_b = Order::new(BidOrAsk::Bid, 100.0);
        
        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);
        assert_eq!(limit.total_volume(), 200.0);
    }

    #[test]
    fn limit_order_multi_fill() {
        let price = dec!(10000.0);
        let mut limit = Limit::new(price);
        let buy_limit_order_a = Order::new(BidOrAsk::Bid, 100.0);
        let buy_limit_order_b = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        let mut market_order_sell = Order::new(BidOrAsk::Ask, 199.0);
        limit.fill_order(&mut market_order_sell);

        assert_eq!(market_order_sell.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().is_filled(), true);
        assert_eq!(limit.orders.get(1).unwrap().is_filled(), false);
        assert_eq!(limit.orders.get(1).unwrap().size, 1.0);

        println!("{:?}", limit);
    }

    #[test]
    fn limit_order_single_fill() {
        let price = dec!(10000.0);
        let mut limit = Limit::new(price);

        let buy_limit_order = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order);

        let mut market_order_sell = Order::new(BidOrAsk::Ask, 99.0);
        limit.fill_order(&mut market_order_sell);

        assert_eq!(market_order_sell.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().size, 1.0);
    }
}