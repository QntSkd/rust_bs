#![allow(non_snake_case)]
mod black_scholes;

fn main() {
    let option = black_scholes::BlackScholes{
        S: 100.00, // spot price
        K: 110.00, // strike price
        r: 0.016, // risk-free rate 1.6%
        t: 0.08333333333, //1 month until expiration
        v: 0.15 // 15% volatility
    };
    println!("{:.4}", option.call_price());
    println!("{:.4}", option.put_price());
}