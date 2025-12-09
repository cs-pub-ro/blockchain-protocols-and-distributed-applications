# 🛠️ Practice: Caching & Scalability Demo

In this session, we will demonstrate how a single caching layer can protect your application from Rate Limits and allow it to scale to hundreds of concurrent users using **NestJS** and **Redis**.

---

## 1. Environment Setup

### Step 1: Clone the Repository
Clone the demo repository containing the NestJS application and the load testing script:
```bash
git clone https://github.com/stefangutica/mx-caching-demo.git
cd mx-caching-demo
```

### Step 2: Start Redis
Use Docker to spin up a local Redis instance.
```bash
docker-compose up -d
```

### Step 3: Install Dependencies
```bash
npm install
```

### Step 4: Configure Load Test Script
Open `load-test.js` and ensure the constants are aggressive:
```javascript
const CONCURRENT_REQUESTS = 100; // High enough to trigger Rate Limit
const DELAY_BETWEEN_BATCHES_MS = 2000;
```

### Step 5: Configure Environment
Create a `.env` file in the root:
```env
# Initial Configuration: Caching is ENABLED
CACHING_ENABLED=true
REDIS_HOST=localhost
REDIS_PORT=6379
```

---

## 2. Experiment A: Scalability (Caching ON)

1.  **Start the Server:**
    ```bash
    npm run start
    ```
2.  **Run Load Test (New Terminal):**
    ```bash
    node load-test.js
    ```

### 🧐 What to observe:
* **Load Test:** `✅ Successful Requests: 100`, `❌ Failed Requests: 0`.
* **Server Logs:** `🌍 CACHE MISS...` appears **ONLY ONCE**.
    * *Explanation:* The first user triggered the request. 99 others got data from Redis.

---

## 3. Experiment B: The Bottleneck (Caching OFF)

1.  **Modify Configuration (.env):**
    ```env
    CACHING_ENABLED=false
    ```
2.  **Restart Server:** Stop (`Ctrl+C`) and start again:
    ```bash
    npm run start
    ```
3.  **Run Load Test:**
    ```bash
    node load-test.js
    ```

### 🧐 What to observe:
* **Server Logs:** `🌍 CACHE MISS...` appears **100 times** rapidly.
* **Load Test:** Likely `429 Too Many Requests` or slower execution time.
    * *Conclusion:* Without caching, you are limited by the provider's API limits.

---

## 4. Experiment C: Data Persistence (Warm Restart)

Demonstrating that Redis keeps data even if the API crashes.

1.  **Enable Caching (.env):**
    ```env
    CACHING_ENABLED=true
    ```
2.  **Increase TTL (src/app.service.ts):**
    ```typescript
    const ttlSeconds = 60; // Increase to 60s
    // Update the getOrSet call:
    return await this.cacheService.getOrSet(
        cacheKey,
        async () => await this.fetchTransactionsFromApi(),
        ttlSeconds, 
    );
    ```
3.  **Start Server & Populate Cache:**
    * `npm run start`
    * Run `node load-test.js` **once**.
    * Check logs: **1** network fetch.
4.  **Crash Simulation:**
    * **Stop server** (`Ctrl+C`).
    * **Start server** immediately.
5.  **Run Load Test Again:**
    * Run `node load-test.js`.

### 🧐 What to observe:
* **Server Logs:** **ZERO** network requests logged.
* *Explanation:* Server restarted ("Warm Start"), found data in Redis, and served it instantly.

---

## 🧹 Cleanup
```bash
docker-compose down
```