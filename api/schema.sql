CREATE TABLE depth_price_history (
  id SERIAL PRIMARY KEY,
  pool TEXT NOT NULL,
  start_time TIMESTAMPTZ NOT NULL,
  end_time TIMESTAMPTZ NOT NULL,
  asset_depth BIGINT NOT NULL,
  rune_depth BIGINT NOT NULL,
  asset_price NUMERIC NOT NULL,
  asset_price_usd NUMERIC NOT NULL,
  liquidity_units BIGINT NOT NULL,
  members_count BIGINT NOT NULL,
  synth_units BIGINT NOT NULL,
  synth_supply BIGINT NOT NULL,
  units BIGINT NOT NULL,
  luvi NUMERIC NOT NULL,
  UNIQUE (pool, start_time, end_time)
);

CREATE TABLE rune_pool_history (
  id SERIAL PRIMARY KEY,
  start_time TIMESTAMPTZ NOT NULL,
  end_time TIMESTAMPTZ NOT NULL,
  count BIGINT NOT NULL,
  units BIGINT NOT NULL,
  UNIQUE (start_time, end_time)
);

CREATE TABLE earnings_history (
  id SERIAL PRIMARY KEY,
  start_time TIMESTAMPTZ NOT NULL,
  end_time TIMESTAMPTZ NOT NULL,
  liquidity_fees BIGINT NOT NULL,
  block_rewards BIGINT NOT NULL,
  earnings BIGINT NOT NULL,
  bonding_earnings BIGINT NOT NULL,
  liquidity_earnings BIGINT NOT NULL,
  avg_node_count NUMERIC NOT NULL,
  rune_price_usd NUMERIC NOT NULL,
  UNIQUE (start_time, end_time)
);

CREATE TABLE pool_earnings (
  id SERIAL PRIMARY KEY,
  earnings_history_id INTEGER NOT NULL REFERENCES earnings_history(id) ON DELETE CASCADE,
  pool TEXT NOT NULL,
  asset_liquidity_fees BIGINT NOT NULL,
  rune_liquidity_fees BIGINT NOT NULL,
  total_liquidity_fees_rune BIGINT NOT NULL,
  saver_earning BIGINT NOT NULL,
  rewards BIGINT NOT NULL,
  earnings BIGINT NOT NULL,
  UNIQUE (earnings_history_id, pool)
);

CREATE TABLE swaps_history (
  id SERIAL PRIMARY KEY,
  pool TEXT NOT NULL,
  start_time TIMESTAMPTZ NOT NULL,
  end_time TIMESTAMPTZ NOT NULL,
  to_asset_count BIGINT NOT NULL,
  to_rune_count BIGINT NOT NULL,
  to_trade_count BIGINT NOT NULL,
  from_trade_count BIGINT NOT NULL,
  synth_mint_count BIGINT NOT NULL,
  synth_redeem_count BIGINT NOT NULL,
  total_count BIGINT NOT NULL,
  to_asset_volume BIGINT NOT NULL,
  to_rune_volume BIGINT NOT NULL,
  to_trade_volume BIGINT NOT NULL,
  from_trade_volume BIGINT NOT NULL,
  synth_mint_volume BIGINT NOT NULL,
  synth_redeem_volume BIGINT NOT NULL,
  total_volume BIGINT NOT NULL,
  to_asset_volume_usd BIGINT NOT NULL,
  to_rune_volume_usd BIGINT NOT NULL,
  to_trade_volume_usd BIGINT NOT NULL,
  from_trade_volume_usd BIGINT NOT NULL,
  synth_mint_volume_usd BIGINT NOT NULL,
  synth_redeem_volume_usd BIGINT NOT NULL,
  total_volume_usd BIGINT NOT NULL,
  to_asset_fees BIGINT NOT NULL,
  to_rune_fees BIGINT NOT NULL,
  to_trade_fees BIGINT NOT NULL,
  from_trade_fees BIGINT NOT NULL,
  synth_mint_fees BIGINT NOT NULL,
  synth_redeem_fees BIGINT NOT NULL,
  total_fees BIGINT NOT NULL,
  to_asset_average_slip NUMERIC NOT NULL,
  to_rune_average_slip NUMERIC NOT NULL,
  to_trade_average_slip NUMERIC NOT NULL,
  from_trade_average_slip NUMERIC NOT NULL,
  synth_mint_average_slip NUMERIC NOT NULL,
  synth_redeem_average_slip NUMERIC NOT NULL,
  average_slip NUMERIC NOT NULL,
  rune_price_usd NUMERIC NOT NULL,
  UNIQUE (pool, start_time, end_time)
);