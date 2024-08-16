use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use uuid::Uuid;
use std::io;
use colored::*;   
use crate::matching_engine::engine::MatchingEngine;
use crate::matching_engine::orderbook::{BidOrAsk, Order, Orderbook};

pub fn run_cli(engine: &mut MatchingEngine) {
    let mut orderbook = Orderbook::new();

    loop {
        display_menu();
        let choice = get_user_input("Choose:");

        match choice.trim() {
            "1" => place_market_order(&mut orderbook),
            "2" => place_limit_order(&mut orderbook),
            "3" => cancel_order(&mut orderbook),
            "4" => orderbook.display(),
            "5" => break,
            _ => println!("{}", "This operation isn't available".red())
        }
    }
}

fn display_menu() {
    println!("\n=== Rust Orderbook ===\n");
    println!("1. Place a market order.");
    println!("2. Place a limit order.");
    println!("3. Cancel an order.");
    println!("4. Print the orderbook.");
    println!("5. Exit.");
}

fn get_user_input(text: &str) -> String {
    println!("{}", text);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}


fn place_market_order(orderbook: &mut Orderbook) {
    let side = get_user_input("Enter a side: \n1. Buy. \n2. Sell.");
    
    let side: u8 = side.parse().unwrap();
    let side = match side {
        1 => BidOrAsk::Bid,
        2 => BidOrAsk::Ask,
        _ => {
            println!("{}", "You can only choose 1 or 2.".red());
            return
        }
    };

    let quantity = get_user_input("Enter a quantity:");
    let quantity: f64 = match quantity.parse() {
        Ok(quantity) => quantity,
        Err(_) => {
            println!("{}", "Invalid quantity.".red());
            return;
        }
    };

    let mut order = Order::new(side, quantity);
    orderbook.fill_market_order(&mut order);
}

fn place_limit_order(orderbook: &mut Orderbook) {
    let side = get_user_input("Enter a side: \n1. Buy. \n2. Sell.");
    let side = match side.trim() {
        "1" => BidOrAsk::Bid,
        "2" => BidOrAsk::Ask,
        _ => {
            println!("{}", "You can only choose 1 or 2.".red());
            return;
        }
    };

    let price = get_user_input("Enter a price:");
    let price: f64 = match price.parse() {
        Ok(price) => price,
        Err(_) => return
    };

    let quantity = get_user_input("Enter a quantity:");
    let quantity: f64 = match quantity.parse() {
        Ok(quantity) => quantity,
        Err(_) => return
    };

    let order = Order::new(side, quantity);

    match Decimal::from_f64(price) {
        Some(price) => {
            orderbook.add_limit_order(price, order);
            println!("{}", "The order has been submitted!".green())
        },
        None => return 
    }
}

fn cancel_order(orderbook: &mut Orderbook) {
    let id = get_user_input("Enter order id:");
    let id = Uuid::parse_str(id.as_str()).unwrap();

    orderbook.cancel_limited_order(id);
    println!("{}", "The order is cancelled!".green());
}

