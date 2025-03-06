-- Depth History
CREATE TABLE depth_history (
    id SERIAL PRIMARY KEY,
    pool VARCHAR NOT NULL,
    asset_depth BIGINT NOT NULL,
    rune_depth BIGINT NOT NULL,
    asset_price DOUBLE PRECISION NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (pool, timestamp)
);

-- Swaps History
CREATE TABLE swaps_history (
    id SERIAL PRIMARY KEY,
    pool VARCHAR NOT NULL,
    from_asset VARCHAR NOT NULL,
    to_asset VARCHAR NOT NULL,
    amount BIGINT NOT NULL,
    fee BIGINT NOT NULL,
    volume_usd DOUBLE PRECISION NOT NULL, -- Trading volume in USD
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (pool, timestamp)
);

-- Earnings History
CREATE TABLE earnings_history (
    id SERIAL PRIMARY KEY,
    pool VARCHAR NOT NULL,
    liquidity_fees BIGINT NOT NULL,
    block_rewards BIGINT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (pool, timestamp)
);

-- RunePool History
CREATE TABLE runepool_history (
    id SERIAL PRIMARY KEY,
    total_units BIGINT NOT NULL,
    members_count INTEGER NOT NULL,
    value BIGINT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (timestamp)
);