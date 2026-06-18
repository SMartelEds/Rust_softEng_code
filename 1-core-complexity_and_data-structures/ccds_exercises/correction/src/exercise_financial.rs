use std::collections::HashMap;

// Define a struct to represent a financial asset
#[derive(Debug)]
struct Asset {
    ticker: String,
    quantity: u32,
    price: f64,
    risk: RiskLevel,
}

// Define an enum to represent the risk level of an asset
#[derive(Debug, PartialEq)]
enum RiskLevel {
    Low,
    Medium,
    High,
}

// Calculate the total value of the portfolio
fn calculate_portfolio_value(assets: &[Asset]) -> f64 {
    assets
        .iter()
        .map(|asset| asset.quantity as f64 * asset.price)
        .sum()
}

// Calculate the diversification score (number of unique tickers)
fn diversification_score(assets: &[Asset]) -> usize {
    let mut tickers = HashMap::new();
    for asset in assets {
        tickers.entry(&asset.ticker).or_insert(());
    }
    tickers.len()
}

// Identify high-risk assets
fn high_risk_assets(assets: &[Asset]) -> Vec<&Asset> {
    assets
        .iter()
        .filter(|asset| matches!(asset.risk, RiskLevel::High))
        .collect()
}

// Filter out assets with a value below the threshold
fn filter_low_value_assets(assets: &[Asset], threshold: f64) -> Vec<&Asset> {
    assets
        .iter()
        .filter(|asset| asset.quantity as f64 * asset.price >= threshold)
        .collect()
}

pub fn exercise_financial_main() {
    // Sample portfolio data
    let portfolio = vec![
        Asset {
            ticker: String::from("AAPL"),
            quantity: 10,
            price: 150.0,
            risk: RiskLevel::Low,
        },
        Asset {
            ticker: String::from("TSLA"),
            quantity: 5,
            price: 700.0,
            risk: RiskLevel::High,
        },
        Asset {
            ticker: String::from("AMZN"),
            quantity: 2,
            price: 3000.0,
            risk: RiskLevel::Medium,
        },
        Asset {
            ticker: String::from("GOOGL"),
            quantity: 3,
            price: 2500.0,
            risk: RiskLevel::Medium,
        },
        Asset {
            ticker: String::from("NVDA"),
            quantity: 4,
            price: 200.0,
            risk: RiskLevel::High,
        },
    ];

    // Calculate and print the total portfolio value
    let total_value = calculate_portfolio_value(&portfolio);
    println!("Total portfolio value: {:.2}", total_value);

    // Calculate and print the diversification score
    let score = diversification_score(&portfolio);
    println!("Diversification score (unique assets): {}", score);

    // Identify and print high-risk assets
    let high_risk = high_risk_assets(&portfolio);
    println!("High-risk assets: {:?}", high_risk);

    // Filter and print assets with a value above the threshold
    let filtered_assets = filter_low_value_assets(&portfolio, 1000.0);
    println!("Assets with value >= 1000.0: {:?}", filtered_assets);
}
