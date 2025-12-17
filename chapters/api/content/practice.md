# Practice: Caching & Scalability Demo

In this session, we will demonstrate how a single caching layer can protect your application from Rate Limits and allow it to scale to hundreds of concurrent users using **NestJS** and **Redis**.

** Learning Objectives:**
1.  See how **Rate Limits** block high-traffic apps.
2.  Visualize how **Caching** reduces 100 network calls to just 1.
3.  Demonstrate **Data Persistence** (Warm Start) where data survives application crashes.

---

## 1. Environment Setup

### Step 1: Clone the Repository
Clone the demo repository containing the NestJS application and the load testing script:
```bash
git clone [https://github.com/stefangutica/mx-caching-demo.git](https://github.com/stefangutica/mx-caching-demo.git)
cd mx-caching-demo
```

### Step 2: Start Redis using Docker
We use Docker to spin up a local Redis instance.
```bash
docker-compose up -d
```

> ** Why do we need Docker?**
> Redis is a database that runs on the operating system. Installing it manually on Windows/MacOS/Linux can be complicated.
> Docker allows us to run Redis in a lightweight "container" instantly, without installing it permanently on your laptop. It acts as our **external shared memory**.

### Step 3: Install Dependencies
Install the required Node.js packages for the server and the script:
```bash
npm install
```

### Step 4: Configure Load Test Script
Open `load-test.js`. This script simulates users refreshing your dApp. We need to tweak the settings to be "aggressive".

```javascript
// Open load-test.js and ensure these values are set:

const CONCURRENT_REQUESTS = 100; 
// Most public APIs (like MultiversX API) allow ~5-10 requests per second. 
// We use 100 to GUARANTEE we hit the limit and crash the request, proving the need for caching.

const DELAY_BETWEEN_BATCHES_MS = 2000; 
// We chose 2 seconds as a suitable delay to give you enough time to read and analyze 
// the logs between batches before the screen scrolls too fast. Adapt it as you wish.
```

### Step 5: Configure Environment
Create a `.env` file in the root of the project:

```env
# Initial Configuration: Caching is ENABLED
CACHING_ENABLED=true
REDIS_HOST=localhost
REDIS_PORT=6379
```

---

## 2. Experiment A: Scalability (Caching ON)

In this scenario, we simulate a healthy, scalable application.

1.  **Start the Server:**
    ```bash
    npm run start
    ```
2.  **Run Load Test (New Terminal):**
    ```bash
    node load-test.js
    ```

### What is happening?
* **The Data:** Your server is asking the MultiversX API for the **latest 25 transactions** on the blockchain.
* **The Traffic:** 100 users are asking for this data at the exact same millisecond.

### What to observe:
* **Load Test:** `Successful Requests: 100`, `Failed Requests: 0`.
* **Server Logs:** `CACHE MISS...` appears **ONLY ONCE**.
    * *Explanation:* The first user triggered the request. The server saved the result in Redis. The subsequent 99 users received the data instantly from Redis (RAM), without touching the MultiversX API.

---

## 3. Experiment B: The Bottleneck (Caching OFF)

Now, let's see why caching is critical by turning it off.

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

### What is happening?
* Now, every single user request forces your server to call the external API. 
* Your server effectively becomes a tool for **DDoS-ing** the external API.

### What to observe:
* **Server Logs:** `CACHE MISS...` appears **100 times** rapidly in a single batch.
* **Load Test:** You will likely see `429 Too Many Requests` errors.
    * *Conclusion:* The external API blocked you because you sent too many requests. Without caching, your app cannot scale beyond a few users.

---

## 4. Experiment C: Data Persistence (Warm Restart)

One huge advantage of Redis (external cache) over local variable caching is that data survives server restarts.

1.  **Enable Caching (.env):**
    ```env
    CACHING_ENABLED=true
    ```
2.  **Increase TTL (src/app.service.ts):**
    Open `src/app.service.ts` and temporarily increase the Time-To-Live (TTL) so the cache doesn't expire while we restart the server.
    ```typescript
    const ttlSeconds = 60; // Increase to 60s
    ```
3.  **Start Server & Populate Cache:**
    * `npm run start`
    * Run `node load-test.js` **once** (let one batch finish).
    * *Check logs:* You see **1** network fetch.
4.  **The "Crash" Simulation:**
    * **Stop the server** (`Ctrl + C`) to simulate a crash or deployment.
    * **Start the server** immediately.
5.  **Run Load Test Again:**
    * Run `node load-test.js`.

### What is happening?
* **Memory Wipe:** When you stop the server (`Ctrl+C`), the Node.js process dies, and any data stored in standard variables/RAM is lost.
* **External Storage:** Redis is running in a separate container (Docker). It stays alive even if your app crashes. When your app restarts, it reconnects to Redis, finds the data still there, and serves it instantly.

### What to observe:
* **Server Logs:** **ZERO** network requests logged.
* *Explanation:* Even though the Node.js process died, the data remained safe in Docker (Redis). When the server restarted ("Warm Start"), it found the data and served it instantly.

---

## Cleanup
Stop the Redis container to free up resources:
```bash
docker-compose down
```