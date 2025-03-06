

# **Catalog Week 3 Assignment: Midgard API Integration**

## **Overview**

This project involved building a Rust-based API to fetch, store, and query historical data from the Midgard API (Nine Realms), deployed on Render. The API supports endpoints for depth and price history, swaps history, earnings history, and RUNEPool history, with advanced query capabilities, scheduled data updates, and Swagger documentation. The journey spanned from initial setup to overcoming deployment and functionality challenges, culminating in a fully operational, testable API.

- **Deployed URL:** `https://catalog-week3-assignment-dwr0.onrender.com/`

---

## **Requirements**

The assignment outlined the following steps:

1. **External Data Source:** Use Midgard API to fetch Depth and Price History, Earnings History, Swaps History, and RUNEPool History.
2. **Database and Schema:** Choose a database (e.g., PostgreSQL) and design a schema for storing fetched data.
3. **API Development:** Build a Rust API with flexible querying, supporting pagination (`page`, `limit`) and advanced parameters (e.g., `date_range`, `liquidity_gt`).
4. **Advanced Query Capabilities:** Enable complex queries, potentially with table joins.
5. **Automated Job:** Implement an hourly job to update data from Midgard API without duplicates.
6. **Swagger/Postman Documentation:** Document endpoints with examples and error cases.
7. **Deployment:** Deploy to a cloud service (e.g., Render) for testing.

---

## **Implementation Journey**

### **Step 1: Project Setup**
- **Objective:** Establish the Rust project structure and dependencies.
- **Actions:**
  - Created a new Rust project: `cargo new api --bin`.
  - Directory: `/mnt/c/Users/shann/Downloads/Catalog-Assignments/catalog-week3-assignment/api`.
  - Added dependencies in `Cargo.toml`:
    ```toml
    [dependencies]
    actix-web = "4"
    deadpool-postgres = "0.10"
    tokio = { version = "1", features = ["full"] }
    tokio-postgres = "0.7"
    reqwest = { version = "0.11", features = ["json"] }
    serde = { version = "1", features = ["derive"] }
    chrono = { version = "0.4", features = ["serde"] }
    dotenv = "0.15"
    env_logger = "0.10"
    log = "0.4"
    tokio-cron-scheduler = "0.9"
    url = "2.5"
    ```
  - Organized modules: `src/main.rs`, `src/db/mod.rs`, `src/fetcher.rs`, `src/jobs/mod.rs`, `src/models.rs`, `src/routes/mod.rs`, `src/services/mod.rs`.

- **Roadblocks:**
  - None initially; setup was straightforward.

---

### **Step 2: Database and Schema Design**
- **Objective:** Set up PostgreSQL and define schemas for Midgard data.
- **Actions:**
  - Chose Render’s PostgreSQL for deployment and local PostgreSQL for testing.
  - Defined schemas in `src/db/mod.rs` (or migrations):
    ```sql
    CREATE TABLE depth_history (
        id SERIAL PRIMARY KEY,
        pool VARCHAR NOT NULL,
        asset_depth BIGINT NOT NULL,
        rune_depth BIGINT NOT NULL,
        asset_price DOUBLE PRECISION NOT NULL,
        timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
        UNIQUE (pool, timestamp)
    );
    CREATE TABLE swaps_history (
        id SERIAL PRIMARY KEY,
        pool VARCHAR NOT NULL,
        from_asset VARCHAR NOT NULL,
        to_asset VARCHAR NOT NULL,
        amount BIGINT NOT NULL,
        fee BIGINT NOT NULL,
        volume_usd DOUBLE PRECISION NOT NULL,
        timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
        UNIQUE (pool, timestamp)
    );
    CREATE TABLE earnings_history (
        id SERIAL PRIMARY KEY,
        pool VARCHAR NOT NULL,
        liquidity_fees BIGINT NOT NULL,
        block_rewards BIGINT NOT NULL,
        timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
        UNIQUE (pool, timestamp)
    );
    CREATE TABLE runepool_history (
        id SERIAL PRIMARY KEY,
        total_units BIGINT NOT NULL,
        members_count INTEGER NOT NULL,
        value BIGINT NOT NULL,
        timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
        UNIQUE (timestamp)
    );
    ```
- **Roadblocks:**
  - **Initial Absence:** Forgot to create tables on Render’s PostgreSQL, leading to "relation does not exist" errors later.
  - **Solution:** Manually ran the above SQL via `psql <internal-database-url>` on Render.

---

### **Step 3: API Development**
- **Objective:** Build RESTful endpoints with query parameters.
- **Actions:**
  - Implemented `src/main.rs` with Actix Web:
    ```rust
    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        let pool = setup_db_pool()?;
        let service = DepthService::new(pool.clone());
        setup_jobs(pool.clone()).await?;
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(service.clone()))
                .configure(config)
        })
        .bind("0.0.0.0:8080")?
        .run()
        .await
    }
    ```
  - Added routes in `src/routes/mod.rs`:
    ```rust
    #[get("/api/depth-history")]
    async fn depth_history(query: web::Query<HistoryQuery>, service: web::Data<DepthService>) -> impl Responder {
        service.get_depths(query.into_inner()).await
            .map(|data| HttpResponse::Ok().json(data))
            .unwrap_or_else(|e| HttpResponse::InternalServerError().body(e.to_string()))
    }
    ```
  - Defined `HistoryQuery` in `src/models.rs`:
    ```rust
    #[derive(Deserialize)]
    pub struct HistoryQuery {
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        liquidity_gt: Option<i64>,
        sort_by: Option<String>,
        order: Option<String>,
        page: Option<i64>,
        limit: Option<i64>,
    }
    ```
- **Roadblocks:**
  - **Parameter Parsing:** Initial errors with invalid query types (e.g., `limit=invalid`).
  - **Solution:** Used `Option` types and validated in `src/services/mod.rs`.

---

### **Step 4: Data Fetching**
- **Objective:** Fetch data from Midgard API.
- **Actions:**
  - Implemented `src/fetcher.rs`:
    ```rust
    pub async fn fetch_depth_data(pool: &Pool, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        let url = "https://midgard.ninerealms.com/v2/history/depths?pool=BTC.BTC&interval=day&count=100";
        let response = client.get(url).send().await?.json::<serde_json::Value>().await?;
        let intervals = response["intervals"].as_array().ok_or("Expected 'intervals'")?;
        let db_client = pool.get().await?;
        for interval in intervals {
            db_client.execute(/* INSERT query */).await?;
        }
        Ok(())
    }
    ```
- **Roadblocks:**
  - **Wrong URLs:** Initially used incorrect endpoints (e.g., `/depths/BTC.BTC`), causing 404 errors.
  - **Solution:** Corrected to query parameter format (e.g., `?pool=BTC.BTC`).
  - **Rate Limiting (429):** Frequent requests hit Midgard’s rate limit.
  - **Solution:** Added retry logic later.

---

### **Step 5: Scheduled Job**
- **Objective:** Automate hourly data updates.
- **Actions:**
  - Used `tokio-cron-scheduler` in `src/jobs/mod.rs`:
    ```rust
    pub async fn setup_jobs(pool: Pool) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let sched = JobScheduler::new().await?;
        let client = reqwest::Client::new();
        let service = DepthService::new(pool.clone());
        let job = Job::new_async("0 0 * * * *", move |_, _| {
            let client = client.clone();
            let service = service.clone();
            Box::pin(async move {
                service.fetch_and_store_depths(&client).await.unwrap_or_else(|e| error!("Depth error: {}", e));
            })
        })?;
        sched.add(job).await?;
        sched.start().await?;
        Ok(())
    }
    ```
- **Roadblocks:**
  - **Empty Database:** Job ran but didn’t insert data due to schema errors.
  - **Solution:** Created tables manually.
  - **429 Errors:** Minute-by-minute testing hit rate limits.
  - **Solution:** Reverted to hourly and added retries.

---

### **Step 6: Deployment on Render**
- **Objective:** Deploy to Render.
- **Actions:**
  - Configured Render:
    - **Build Command:** `cargo build --release`
    - **Start Command:** `./target/release/api`
    - **Environment Variables:** `DATABASE_URL`, `RUST_LOG=info`.
  - Pushed to GitHub and linked to Render.
- **Roadblocks:**
  - **Binary Missing:** Initial builds failed to produce `target/release/api`.
  - **Solution:** Ensured `cargo build --release` and correct binary name (`api`).
  - **Port Binding:** Render couldn’t detect the port.
  - **Solution:** Bound to `0.0.0.0:$PORT` in `main.rs`.
  - **CORS:** Swagger testing failed due to CORS restrictions.
  - **Solution:** Added `actix-cors`.

---

### **Step 7: Advanced Query Capabilities**
- **Objective:** Support joins and complex queries.
- **Actions:**
  - Added `/api/pool-activity/{pool_id}` in `src/routes/mod.rs`:
    ```rust
    #[get("/api/pool-activity/{pool_id}")]
    async fn pool_activity(path: web::Path<String>, query: web::Query<PoolActivityQuery>, service: web::Data<DepthService>) -> impl Responder {
        let pool_id = path.into_inner();
        service.get_pool_activity(pool_id, query.start_date, query.end_date, query.page.unwrap_or(1), query.limit.unwrap_or(10)).await
            .map(|data| HttpResponse::Ok().json(data))
            .unwrap_or_else(|e| HttpResponse::InternalServerError().body(e.to_string()))
    }
    ```
  - Implemented join in `src/db/mod.rs`:
    ```rust
    pub async fn get_pool_activity(pool: &Pool, pool_id: &str, start_date: Option<DateTime<Utc>>, end_date: Option<DateTime<Utc>>, limit: i64, offset: i64) -> Result<Vec<PoolActivity>, tokio_postgres::Error> {
        let client = pool.get().await?;
        let query = "SELECT d.pool, d.asset_depth, d.rune_depth, d.asset_price, COALESCE(s.amount, 0) as swap_amount, COALESCE(s.fee, 0) as swap_fee, COALESCE(s.volume_usd, 0.0) as volume_usd, d.timestamp
                     FROM depth_history d LEFT JOIN swaps_history s ON d.pool = s.pool AND d.timestamp = s.timestamp
                     WHERE d.pool = $1 AND ($2::timestamp IS NULL OR d.timestamp >= $2) AND ($3::timestamp IS NULL OR d.timestamp <= $3)
                     ORDER BY d.timestamp DESC LIMIT $4 OFFSET $5";
        let rows = client.query(query, &[&pool_id, &start_date, &end_date, &limit, &offset]).await?;
        Ok(rows.into_iter().map(|row| PoolActivity { /* fields */ }).collect())
    }
    ```
- **Roadblocks:**
  - **Join Complexity:** Initially unsure how to join without normalization.
  - **Solution:** Used `LEFT JOIN` on existing tables.

---

### **Step 8: Swagger Documentation**
- **Objective:** Document all endpoints.
- **Actions:**
  - Created `swagger.yaml`:
    ```yaml
    openapi: 3.0.0
    servers:
      - url: https://catalog-week3-assignment-dwr0.onrender.com/
    paths:
      /api/depth-history:
        get:
          summary: Retrieve depth and price history data
          parameters:
            - name: start_date
              in: query
              schema: { type: string, format: date-time }
            # ... other endpoints ...
      /api/pool-activity/{pool_id}:
        get:
          summary: Retrieve combined pool activity data
          # ... details ...
    components:
      schemas:
        Depth: { type: object, properties: { pool: { type: string }, /* ... */ } }
        # ... other schemas ...
    ```
- **Roadblocks:**
  - **CORS Error:** Swagger Editor couldn’t fetch due to CORS.
  - **Solution:** Added CORS in `main.rs`.
  - **Alignment Issues:** YAML indentation errors.
  - **Solution:** Fixed with consistent 2-space indentation.

---

### **Step 9: Rate Limiting and 429 Handling**
- **Objective:** Enhance robustness (recommended).
- **Actions:**
  - Updated `src/fetcher.rs`:
    ```rust
    pub async fn fetch_depth_data(pool: &Pool, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        let url = "https://midgard.ninerealms.com/v2/history/depths?pool=BTC.BTC&interval=day&count=100";
        let mut attempts = 0;
        let response = loop {
            let resp = client.get(url).send().await?;
            if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                attempts += 1;
                if attempts >= 3 { return Err("Max retries".into()); }
                sleep(Duration::from_secs(2u64.pow(attempts as u32))).await;
            } else if !resp.status().is_success() {
                return Err(format!("HTTP {}", resp.status()).into());
            } else { break resp; }
        };
        // ... insert data ...
        Ok(())
    }
    ```
- **Roadblocks:**
  - **Frequent 429s:** Minute-by-minute job hit limits.
  - **Solution:** Reverted to hourly + retries.

---

## **Testing Procedures**

### **Local Testing**
- **Command:** `RUST_LOG=info cargo run`
- **Unit Tests:** In `src/main.rs`:
  ```rust
  #[actix_rt::test]
  async fn test_all_endpoints() {
      let pool = /* setup */;
      let app = test::init_service(App::new().app_data(web::Data::new(DepthService::new(pool))).configure(config)).await;
      for endpoint in ["/api/depth-history", /* ... */] {
          let req = test::TestRequest::get().uri(&format!("{}?limit=5", endpoint)).to_request();
          let resp = test::call_service(&app, req).await;
          assert!(resp.status().is_success());
      }
  }
  ```

### **CURL Testing**
- **Basic:**
  ```bash
  curl "https://catalog-week3-assignment-dwr0.onrender.com/api/depth-history?limit=5"
  ```
- **Advanced:**
  ```bash
  curl "https://catalog-week3-assignment-dwr0.onrender.com/api/pool-activity/BTC.BTC?start_date=2023-08-01T00:00:00Z&limit=5"
  ```

### **Swagger Testing**
- **Steps:**
  1. Load `swagger.yaml` in [Swagger Editor](https://editor.swagger.io/).
  2. Test `/api/depth-history` with `limit=5`, expect 200 with JSON.
  3. Test `/api/pool-activity/BTC.BTC` with multiple parameters.

---

## **Final Outcome**
- **Deployed API:** Fully functional at `https://catalog-week3-assignment-dwr0.onrender.com/`.
- **Features:** All endpoints, advanced queries with joins, hourly data updates, CORS-enabled, Swagger-documented.
- **Lessons Learned:** Importance of CORS, rate limit handling, and thorough logging.

---
