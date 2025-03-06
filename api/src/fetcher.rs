use reqwest::Client;
use deadpool_postgres::Pool;
use crate::models::{Depth, Swap, Earning, RunePool};
use chrono::{DateTime, Utc};
use std::time::Duration;
use tokio::time::sleep;
use log::{info, error};

pub async fn fetch_depth_data(pool: &Pool, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://midgard.ninerealms.com/v2/history/depths?pool=BTC.BTC&interval=day&count=100";
    let mut attempts = 0;
    let max_attempts = 3;

    let response = loop {
        let resp = client.get(url).send().await?;
        match resp.status() {
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                attempts += 1;
                if attempts >= max_attempts {
                    return Err("Max retry attempts reached for depth data".into());
                }
                let delay = Duration::from_secs(2u64.pow(attempts as u32));
                error!("429 Too Many Requests for depth data, retrying in {}s", delay.as_secs());
                sleep(delay).await;
            }
            status if !status.is_success() => {
                return Err(format!("Failed to fetch depth data: HTTP {}", status).into());
            }
            _ => break resp,
        }
    };

    let json: serde_json::Value = response.json().await?;
    let intervals = json["intervals"].as_array().ok_or("Expected 'intervals' array")?;
    let db_client = pool.get().await?;
    for interval in intervals {
        let depth = Depth { /* ... */ };
        db_client.execute(/* ... */).await?;
    }
    info!("Depth data inserted into database");
    Ok(())
}
pub async fn fetch_swaps_data(pool: &Pool, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://midgard.ninerealms.com/v2/history/swaps?pool=BTC.BTC&interval=day&count=100";
    let response = client.get(url).send().await?;
    if !response.status().is_success() {
        return Err(format!("Failed to fetch swaps data: HTTP {}", response.status()).into());
    }
    let json: serde_json::Value = response.json().await?;
    let intervals = json["intervals"].as_array().ok_or("Expected 'intervals' array")?;
    let db_client = pool.get().await?;
    for interval in intervals {
        let swap = Swap {
            pool: "BTC.BTC".to_string(),
            from_asset: "BTC".to_string(), // Simplified, adjust based on API response
            to_asset: "RUNE".to_string(),
            amount: interval["totalVolume"].as_str().unwrap_or("0").parse()?,
            fee: interval["totalFees"].as_str().unwrap_or("0").parse()?,
            volume_usd: interval["totalVolumeUSD"].as_f64().unwrap_or(0.0),
            timestamp: DateTime::from_timestamp(interval["endTime"].as_str().unwrap_or("0").parse::<i64>()? / 1000, 0)
                .unwrap_or(DateTime::<Utc>::MIN_UTC),
        };
        db_client.execute(
            "INSERT INTO swaps_history (pool, from_asset, to_asset, amount, fee, volume_usd, timestamp) 
             VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (pool, timestamp) DO NOTHING",
            &[&swap.pool, &swap.from_asset, &swap.to_asset, &swap.amount, &swap.fee, &swap.volume_usd, &swap.timestamp],
        ).await?;
    }
    Ok(())
}

pub async fn fetch_earnings_data(pool: &Pool, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://midgard.ninerealms.com/v2/history/earnings?interval=day&count=100";
    let response = client.get(url).send().await?;
    if !response.status().is_success() {
        return Err(format!("Failed to fetch earnings data: HTTP {}", response.status()).into());
    }
    let json: serde_json::Value = response.json().await?;
    let intervals = json["intervals"].as_array().ok_or("Expected 'intervals' array")?;
    let db_client = pool.get().await?;
    for interval in intervals {
        let earning = Earning {
            pool: "BTC.BTC".to_string(), // Simplified, adjust based on API response
            liquidity_fees: interval["liquidityFees"].as_str().unwrap_or("0").parse()?,
            block_rewards: interval["blockRewards"].as_str().unwrap_or("0").parse()?,
            timestamp: DateTime::from_timestamp(interval["endTime"].as_str().unwrap_or("0").parse::<i64>()? / 1000, 0)
                .unwrap_or(DateTime::<Utc>::MIN_UTC),
        };
        db_client.execute(
            "INSERT INTO earnings_history (pool, liquidity_fees, block_rewards, timestamp) 
             VALUES ($1, $2, $3, $4) ON CONFLICT (pool, timestamp) DO NOTHING",
            &[&earning.pool, &earning.liquidity_fees, &earning.block_rewards, &earning.timestamp],
        ).await?;
    }
    Ok(())
}

pub async fn fetch_runepool_data(pool: &Pool, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://midgard.ninerealms.com/v2/history/runepool?interval=day&count=100";
    let response = client.get(url).send().await?;
    if !response.status().is_success() {
        return Err(format!("Failed to fetch runepool data: HTTP {}", response.status()).into());
    }
    let json: serde_json::Value = response.json().await?;
    let intervals = json["intervals"].as_array().ok_or("Expected 'intervals' array")?;
    let db_client = pool.get().await?;
    for interval in intervals {
        let runepool = RunePool {
            total_units: interval["units"].as_str().unwrap_or("0").parse()?,
            members_count: interval["memberCount"].as_str().unwrap_or("0").parse()?,
            value: interval["value"].as_str().unwrap_or("0").parse()?,
            timestamp: DateTime::from_timestamp(interval["endTime"].as_str().unwrap_or("0").parse::<i64>()? / 1000, 0)
                .unwrap_or(DateTime::<Utc>::MIN_UTC),
        };
        db_client.execute(
            "INSERT INTO runepool_history (total_units, members_count, value, timestamp) 
             VALUES ($1, $2, $3, $4) ON CONFLICT (timestamp) DO NOTHING",
            &[&runepool.total_units, &runepool.members_count, &runepool.value, &runepool.timestamp],
        ).await?;
    }
    Ok(())
}