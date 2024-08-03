mod matching_engine;
use matching_engine::orderbook::{BidOrAsk, Order, Orderbook};
use matching_engine::engine::{MatchingEngine, TradingPair};
use rust_decimal_macros::dec;

fn main() {
    let buy_order_ali = Order::new(BidOrAsk::Bid, 100.0);
    let buy_order_bob = Order::new(BidOrAsk::Bid, 235.5);

    let sell_order_a = Order::new(BidOrAsk::Ask, 65.5);
    let sell_order_b = Order::new(BidOrAsk::Ask, 100.7);
    let sell_order_c = Order::new(BidOrAsk::Ask, 632.1);

    let mut orderbook = Orderbook::new();
    orderbook.add_limit_order(dec!(18), buy_order_ali);
    orderbook.add_limit_order(dec!(18.4), buy_order_bob);
    orderbook.add_limit_order(dec!(20.0), sell_order_a);
    orderbook.add_limit_order(dec!(20.5), sell_order_b);
    orderbook.add_limit_order(dec!(22.0), sell_order_c);

    orderbook.display();

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USDT".to_string());
    engine.add_new_market(pair.clone());

    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    // let eth_pair = TradingPair::new("ETH".to_string(), "USDT".to_string());
    engine.place_limit_order(pair, dec!(10.000), buy_order).unwrap();
}
