use reqwest::Client;
use deadpool_postgres::Pool;
use crate::models::{DepthPrice, Swap, Earnings, PoolEarning, RunePool};
use chrono::{DateTime, Utc};
use std::time::Duration;
use tokio::time::sleep;

pub async fn fetch_depth_data(pool: &Pool, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://midgard.ninerealms.com/v2/history/depths/BTC.BTC?interval=day&count=400";
    let mut attempts = 0;
    let max_attempts = 3;
    let response = loop {
        let resp = client.get(url).send().await?;
        if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            attempts += 1;
            if attempts >= max_attempts {
                return Err(format!("Failed to fetch depth data after {} attempts", max_attempts).into());
            }
            sleep(Duration::from_secs(2u64.pow(attempts as u32))).await;
        } else if !resp.status().is_success() {
            return Err(format!("Failed to fetch depth data: HTTP {}", resp.status()).into());
        } else {
            break resp;
        }
    };
    let json: serde_json::Value = response.json().await?;
    let intervals = json["intervals"].as_array().ok_or("Expected 'intervals' array")?;
    let db_client = pool.get().await?;
    for interval in intervals {
        let depth = DepthPrice {
            id: 0, // Assigned by DB
            pool: "BTC.BTC".to_string(),
            start_time: DateTime::from_timestamp(interval["startTime"].as_str().unwrap_or("0").parse::<i64>()? / 1000, 0).unwrap_or(DateTime::<Utc>::MIN_UTC),
            end_time: DateTime::from_timestamp(interval["endTime"].as_str().unwrap_or("0").parse::<i64>()? / 1000, 0).unwrap_or(DateTime::<Utc>::MIN_UTC),
            asset_depth: interval["assetDepth"].as_str().unwrap_or("0").parse()?,
            rune_depth: interval["runeDepth"].as_str().unwrap_or("0").parse()?,
            asset_price: interval["assetPrice"].as_str().unwrap_or("0").parse()?,
            asset_price_usd: interval["assetPriceUSD"].as_str().unwrap_or("0").parse()?,
            liquidity_units: interval["liquidityUnits"].as_str().unwrap_or("0").parse()?,
            members_count: interval["membersCount"].as_str().unwrap_or("0").parse()?,
            synth_units: interval["synthUnits"].as_str().unwrap_or("0").parse()?,
            synth_supply: interval["synthSupply"].as_str().unwrap_or("0").parse()?,
            units: interval["units"].as_str().unwrap_or("0").parse()?,
            luvi: interval["luvi"].as_str().unwrap_or("0").parse()?,
        };
        db_client.execute(
            "INSERT INTO depth_price_history (pool, start_time, end_time, asset_depth, rune_depth, asset_price, asset_price_usd, liquidity_units, members_count, synth_units, synth_supply, units, luvi)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) ON CONFLICT (pool, start_time, end_time) DO NOTHING",
            &[&depth.pool, &depth.start_time, &depth.end_time, &depth.asset_depth, &depth.rune_depth, &depth.asset_price, &depth.asset_price_usd, &depth.liquidity_units, &depth.members_count, &depth.synth_units, &depth.synth_supply, &depth.units, &depth.luvi],
        ).await?;
    }
    Ok(())
}

pub async fn fetch_swaps_data(pool: &Pool, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://midgard.ninerealms.com/v2/history/swaps?interval=day&count=400";
    let mut attempts = 0;
    let max_attempts = 3;
    let response = loop {
        let resp = client.get(url).send().await?;
        if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            attempts += 1;
            if attempts >= max_attempts {
                return Err(format!("Failed to fetch swaps data after {} attempts", max_attempts).into());
            }
            sleep(Duration::from_secs(2u64.pow(attempts as u32))).await;
        } else if !resp.status().is_success() {
            return Err(format!("Failed to fetch swaps data: HTTP {}", resp.status()).into());
        } else {
            break resp;
        }
    };
    let json: serde_json::Value = response.json().await?;
    let intervals = json["intervals"].as_array().ok_or("Expected 'intervals' array")?;
    let db_client = pool.get().await?;
    for interval in intervals {
        let swap = Swap {
            id: 0, // Assigned by DB
            pool: "BTC.BTC".to_string(),
            start_time: DateTime::from_timestamp(interval["startTime"].as_str().unwrap_or("0").parse::<i64>()? / 1000, 0).unwrap_or(DateTime::<Utc>::MIN_UTC),
            end_time: DateTime::from_timestamp(interval["endTime"].as_str().unwrap_or("0").parse::<i64>()? / 1000, 0).unwrap_or(DateTime::<Utc>::MIN_UTC),
            to_asset_count: interval["toAssetCount"].as_str().unwrap_or("0").parse()?,
            to_rune_count: interval["toRuneCount"].as_str().unwrap_or("0").parse()?,
            to_trade_count: interval["toTradeCount"].as_str().unwrap_or("0").parse()?,
            from_trade_count: interval["fromTradeCount"].as_str().unwrap_or("0").parse()?,
            synth_mint_count: interval["synthMintCount"].as_str().unwrap_or("0").parse()?,
            synth_redeem_count: interval["synthRedeemCount"].as_str().unwrap_or("0").parse()?,
            total_count: interval["totalCount"].as_str().unwrap_or("0").parse()?,
            to_asset_volume: interval["toAssetVolume"].as_str().unwrap_or("0").parse()?,
            to_rune_volume: interval["toRuneVolume"].as_str().unwrap_or("0").parse()?,
            to_trade_volume: interval["toTradeVolume"].as_str().unwrap_or("0").parse()?,
            from_trade_volume: interval["fromTradeVolume"].as_str().unwrap_or("0").parse()?,
            synth_mint_volume: interval["synthMintVolume"].as_str().unwrap_or("0").parse()?,
            synth_redeem_volume: interval["synthRedeemVolume"].as_str().unwrap_or("0").parse()?,
            total_volume: interval["totalVolume"].as_str().unwrap_or("0").parse()?,
            to_asset_volume_usd: interval["toAssetVolumeUSD"].as_str().unwrap_or("0").parse()?,
            to_rune_volume_usd: interval["toRuneVolumeUSD"].as_str().unwrap_or("0").parse()?,
            to_trade_volume_usd: interval["toTradeVolumeUSD"].as_str().unwrap_or("0").parse()?,
            from_trade_volume_usd: interval["fromTradeVolumeUSD"].as_str().unwrap_or("0").parse()?,
            synth_mint_volume_usd: interval["synthMintVolumeUSD"].as_str().unwrap_or("0").parse()?,
            synth_redeem_volume_usd: interval["synthRedeemVolumeUSD"].as_str().unwrap_or("0").parse()?,
            total_volume_usd: interval["totalVolumeUSD"].as_str().unwrap_or("0").parse()?,
            to_asset_fees: interval["toAssetFees"].as_str().unwrap_or("0").parse()?,
            to_rune_fees: interval["toRuneFees"].as_str().unwrap_or("0").parse()?,
            to_trade_fees: interval["toTradeFees"].as_str().unwrap_or("0").parse()?,
            from_trade_fees: interval["fromTradeFees"].as_str().unwrap_or("0").parse()?,
            synth_mint_fees: interval["synthMintFees"].as_str().unwrap_or("0").parse()?,
            synth_redeem_fees: interval["synthRedeemFees"].as_str().unwrap_or("0").parse()?,
            total_fees: interval["totalFees"].as_str().unwrap_or("0").parse()?,
            to_asset_average_slip: interval["toAssetAverageSlip"].as_str().unwrap_or("0").parse()?,
            to_rune_average_slip: interval["toRuneAverageSlip"].as_str().unwrap_or("0").parse()?,
            to_trade_average_slip: interval["toTradeAverageSlip"].as_str().unwrap_or("0").parse()?,
            from_trade_average_slip: interval["fromTradeAverageSlip"].as_str().unwrap_or("0").parse()?,
            synth_mint_average_slip: interval["synthMintAverageSlip"].as_str().unwrap_or("0").parse()?,
            synth_redeem_average_slip: interval["synthRedeemAverageSlip"].as_str().unwrap_or("0").parse()?,
            average_slip: interval["averageSlip"].as_str().unwrap_or("0").parse()?,
            rune_price_usd: interval["runePriceUSD"].as_str().unwrap_or("0").parse()?,
        };
        db_client.execute(
            "INSERT INTO swaps_history (pool, start_time, end_time, to_asset_count, to_rune_count, to_trade_count, from_trade_count, synth_mint_count, synth_redeem_count, total_count, to_asset_volume, to_rune_volume, to_trade_volume, from_trade_volume, synth_mint_volume, synth_redeem_volume, total_volume, to_asset_volume_usd, to_rune_volume_usd, to_trade_volume_usd, from_trade_volume_usd, synth_mint_volume_usd, synth_redeem_volume_usd, total_volume_usd, to_asset_fees, to_rune_fees, to_trade_fees, from_trade_fees, synth_mint_fees, synth_redeem_fees, total_fees, to_asset_average_slip, to_rune_average_slip, to_trade_average_slip, from_trade_average_slip, synth_mint_average_slip, synth_redeem_average_slip, average_slip, rune_price_usd)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38, $39)
             ON CONFLICT (pool, start_time, end_time) DO NOTHING",
            &[&swap.pool, &swap.start_time, &swap.end_time, &swap.to_asset_count, &swap.to_rune_count, &swap.to_trade_count, &swap.from_trade_count, &swap.synth_mint_count, &swap.synth_redeem_count, &swap.total_count, &swap.to_asset_volume, &swap.to_rune_volume, &swap.to_trade_volume, &swap.from_trade_volume, &swap.synth_mint_volume, &swap.synth_redeem_volume, &swap.total_volume, &swap.to_asset_volume_usd, &swap.to_rune_volume_usd, &swap.to_trade_volume_usd, &swap.from_trade_volume_usd, &swap.synth_mint_volume_usd, &swap.synth_redeem_volume_usd, &swap.total_volume_usd, &swap.to_asset_fees, &swap.to_rune_fees, &swap.to_trade_fees, &swap.from_trade_fees, &swap.synth_mint_fees, &swap.synth_redeem_fees, &swap.total_fees, &swap.to_asset_average_slip, &swap.to_rune_average_slip, &swap.to_trade_average_slip, &swap.from_trade_average_slip, &swap.synth_mint_average_slip, &swap.synth_redeem_average_slip, &swap.average_slip, &swap.rune_price_usd],
        ).await?;
    }
    Ok(())
}

pub async fn fetch_earnings_data(pool: &Pool, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://midgard.ninerealms.com/v2/history/earnings?interval=day&count=400";
    let mut attempts = 0;
    let max_attempts = 3;
    let response = loop {
        let resp = client.get(url).send().await?;
        if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            attempts += 1;
            if attempts >= max_attempts {
                return Err(format!("Failed to fetch earnings data after {} attempts", max_attempts).into());
            }
            sleep(Duration::from_secs(2u64.pow(attempts as u32))).await;
        } else if !resp.status().is_success() {
            return Err(format!("Failed to fetch earnings data: HTTP {}", resp.status()).into());
        } else {
            break resp;
        }
    };
    let json: serde_json::Value = response.json().await?;
    let intervals = json["intervals"].as_array().ok_or("Expected 'intervals' array")?;
    let db_client = pool.get().await?;
    for interval in intervals {
        let earnings = Earnings {
            id: 0, // Assigned by DB
            start_time: DateTime::from_timestamp(interval["startTime"].as_str().unwrap_or("0").parse::<i64>()? / 1000, 0).unwrap_or(DateTime::<Utc>::MIN_UTC),
            end_time: DateTime::from_timestamp(interval["endTime"].as_str().unwrap_or("0").parse::<i64>()? / 1000, 0).unwrap_or(DateTime::<Utc>::MIN_UTC),
            liquidity_fees: interval["liquidityFees"].as_str().unwrap_or("0").parse()?,
            block_rewards: interval["blockRewards"].as_str().unwrap_or("0").parse()?,
            earnings: interval["earnings"].as_str().unwrap_or("0").parse()?,
            bonding_earnings: interval["bondingEarnings"].as_str().unwrap_or("0").parse()?,
            liquidity_earnings: interval["liquidityEarnings"].as_str().unwrap_or("0").parse()?,
            avg_node_count: interval["avgNodeCount"].as_str().unwrap_or("0").parse()?,
            rune_price_usd: interval["runePriceUSD"].as_str().unwrap_or("0").parse()?,
        };
        let row = db_client.query_one(
            "INSERT INTO earnings_history (start_time, end_time, liquidity_fees, block_rewards, earnings, bonding_earnings, liquidity_earnings, avg_node_count, rune_price_usd)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) ON CONFLICT (start_time, end_time) DO UPDATE SET liquidity_fees = EXCLUDED.liquidity_fees RETURNING id",
            &[&earnings.start_time, &earnings.end_time, &earnings.liquidity_fees, &earnings.block_rewards, &earnings.earnings, &earnings.bonding_earnings, &earnings.liquidity_earnings, &earnings.avg_node_count, &earnings.rune_price_usd],
        ).await?;
        let earnings_id: i32 = row.get("id");

        if let Some(pools) = interval["pools"].as_array() {
            for pool_data in pools {
                let pool_earning = PoolEarning {
                    id: 0, // Assigned by DB
                    earnings_history_id: earnings_id,
                    pool: pool_data["pool"].as_str().unwrap_or("BTC.BTC").to_string(),
                    asset_liquidity_fees: pool_data["assetLiquidityFees"].as_str().unwrap_or("0").parse()?,
                    rune_liquidity_fees: pool_data["runeLiquidityFees"].as_str().unwrap_or("0").parse()?,
                    total_liquidity_fees_rune: pool_data["totalLiquidityFeesRune"].as_str().unwrap_or("0").parse()?,
                    saver_earning: pool_data["saverEarning"].as_str().unwrap_or("0").parse()?,
                    rewards: pool_data["rewards"].as_str().unwrap_or("0").parse()?,
                    earnings: pool_data["earnings"].as_str().unwrap_or("0").parse()?,
                };
                db_client.execute(
                    "INSERT INTO pool_earnings (earnings_history_id, pool, asset_liquidity_fees, rune_liquidity_fees, total_liquidity_fees_rune, saver_earning, rewards, earnings)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT (earnings_history_id, pool) DO NOTHING",
                    &[&pool_earning.earnings_history_id, &pool_earning.pool, &pool_earning.asset_liquidity_fees, &pool_earning.rune_liquidity_fees, &pool_earning.total_liquidity_fees_rune, &pool_earning.saver_earning, &pool_earning.rewards, &pool_earning.earnings],
                ).await?;
            }
        }
    }
    Ok(())
}

pub async fn fetch_runepool_data(pool: &Pool, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://midgard.ninerealms.com/v2/history/runepool?interval=day&count=400";
    let mut attempts = 0;
    let max_attempts = 3;
    let response = loop {
        let resp = client.get(url).send().await?;
        if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            attempts += 1;
            if attempts >= max_attempts {
                return Err(format!("Failed to fetch runepool data after {} attempts", max_attempts).into());
            }
            sleep(Duration::from_secs(2u64.pow(attempts as u32))).await;
        } else if !resp.status().is_success() {
            return Err(format!("Failed to fetch runepool data: HTTP {}", resp.status()).into());
        } else {
            break resp;
        }
    };
    let json: serde_json::Value = response.json().await?;
    let intervals = json["intervals"].as_array().ok_or("Expected 'intervals' array")?;
    let db_client = pool.get().await?;
    for interval in intervals {
        let runepool = RunePool {
            id: 0, // Assigned by DB
            start_time: DateTime::from_timestamp(interval["startTime"].as_str().unwrap_or("0").parse::<i64>()? / 1000, 0).unwrap_or(DateTime::<Utc>::MIN_UTC),
            end_time: DateTime::from_timestamp(interval["endTime"].as_str().unwrap_or("0").parse::<i64>()? / 1000, 0).unwrap_or(DateTime::<Utc>::MIN_UTC),
            count: interval["memberCount"].as_str().unwrap_or("0").parse()?,
            units: interval["units"].as_str().unwrap_or("0").parse()?,
        };
        db_client.execute(
            "INSERT INTO rune_pool_history (start_time, end_time, count, units)
             VALUES ($1, $2, $3, $4) ON CONFLICT (start_time, end_time) DO NOTHING",
            &[&runepool.start_time, &runepool.end_time, &runepool.count, &runepool.units],
        ).await?;
    }
    Ok(())
}