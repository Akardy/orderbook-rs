mod matching_engine;
use matching_engine::orderbook::{BidOrAsk, Order, Orderbook};
use matching_engine::engine::{MatchingEngine, TradingPair};

fn main() {
    let buy_order_ali = Order::new(BidOrAsk::Bid, 5.5);
    //let sell_order = Order::new(BidOrAsk::Ask, 2.45);
    let buy_order_bob = Order::new(BidOrAsk::Bid, 5.5);

    let sell_order = Order::new(BidOrAsk::Ask, 6.5);

    let mut orderbook = Orderbook::new();
    orderbook.add_order(4.4, buy_order_ali);
    orderbook.add_order(4.4, buy_order_bob);
    orderbook.add_order(20.0, sell_order);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USDT".to_string());
    engine.add_new_market(pair.clone());

    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    // let eth_pair = TradingPair::new("ETH".to_string(), "USDT".to_string());
    engine.place_limit_order(pair, 10.000, buy_order).unwrap();
}
