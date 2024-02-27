use binance::api::*;
use binance::market::*;
use binance::account::*;
use std::thread;
use std::time::Duration;
use binance::errors::ErrorKind as BinanceLibErrorKind;

const API_KEY: &str = "YOUR_API_KEY";
const SECRET_KEY: &str = "YOUR_SECRET_KEY";
const COIN: &str = "BTCUSDT";
const BUY_THRESHOLD: f64 = 50000.0; // Example threshold for buying
const SELL_THRESHOLD: f64 = 52000.0; // Example threshold for selling
const QUANTITY: f64 = 0.001; // Example quantity for trading

fn main() {
    let market: Market = Binance::new(Some(API_KEY.into()), Some(SECRET_KEY.into()));
    let account: Account = Binance::new(Some(API_KEY.into()), Some(SECRET_KEY.into()));

    loop {
        match market.get_price(COIN) {
            Ok(price) => {
                println!("Current price for {}: {}", COIN, price.price);
                if price.price <= BUY_THRESHOLD {
                    match account.market_buy(COIN, QUANTITY) {
                        Ok(response) => println!("Bought {} at market price: {:?}", COIN, response),
                        Err(err) => handle_error(err),
                    }
                } else if price.price >= SELL_THRESHOLD {
                    match account.market_sell(COIN, QUANTITY) {
                        Ok(response) => println!("Sold {} at market price: {:?}", COIN, response),
                        Err(err) => handle_error(err),
                    }
                }
            }
            Err(err) => handle_error(err),
        }

        // Wait for some time before checking again
        thread::sleep(Duration::from_secs(10));
    }
}

fn handle_error(err: binance::errors::Error) {
    match err {
        binance::errors::Error(BinanceLibErrorKind::BinanceError(response), _) => {
            match response.code {
                -1013_i16 => println!("Filter failure: LOT_SIZE!"),
                -2010_i16 => println!("Funds insufficient! {}", response.msg),
                _ => println!("Non-catched code {}: {}", response.code, response.msg),
            }
        }
        binance::errors::Error(BinanceLibErrorKind::Msg(msg), _) => println!("Binancelib error msg: {}", msg),
        _ => println!("Other errors: {}.", err),
    }
}
