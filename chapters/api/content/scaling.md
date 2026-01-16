# Scaling your APIs

When building a production-grade dApp or backend service on MultiversX, relying solely on direct calls to the public API is rarely sufficient. As your user base grows, you will encounter performance bottlenecks and restrictions.

## 1. The Limitations of Public APIs

Public endpoints like `api.multiversx.com` or `devnet-api.multiversx.com` are shared resources. To ensure fairness and stability for everyone, they enforce strict rules:

* **Rate Limits:** Public APIs have strict request limits (e.g., a specific number of requests per second per IP). If your dApp exceeds this, you will receive `429 Too Many Requests` errors, breaking the functionality for your users.
* **Network Latency:** Making an HTTP request over the internet to the API for every single piece of data (even static data) is inefficient. It adds unnecessary latency to your application.
* **Redundant Computation:** Querying the same data repeatedly (e.g., "What is the token identifier for this project?") wastes resources.

## 2. In-Memory Caching

The first step to solving these issues is **Caching**. Instead of fetching data from the MultiversX API every time a user requests it, you fetch it once and store it in your server's memory (RAM).

For subsequent requests, you serve the data directly from memory. This acts as a buffer, protecting the public API from spam and providing near-instant responses to your users.

## 3. Shared Memory with Redis

Modern backends often run on multiple instances (containers/servers) to handle load. To manage this efficiently, we use **Redis** as a distributed cache key-value store.

Redis provides two critical advantages:

1.  **Shared State:** If you use simple local variables, **Instance A** won't know what **Instance B** has cached. Redis acts as a central store accessible by all instances, ensuring consistency.
2.  **Persistence & Fast Recovery:** In a production environment, API services restart frequently (due to deployments or autoscaling). When a service restarts, its local RAM is wiped ("cold start"). However, **Redis retains the data**. This means your API boots up with a "warm cache," ready to serve requests immediately without needing to hammer the public API to repopulate data.

### The Architecture

1.  **App receives request:** "Get Smart Contract Config".
2.  **Check Redis:** Does the key `sc:config` exist?
    * **YES:** Return data immediately (0ms latency, 0 API calls).
    * **NO:** Fetch from `api.multiversx.com`, save to Redis with an expiration time (TTL), and return to user.

## 4. MultiversX Cache SDK

To simplify the integration of caching patterns within the ecosystem, specifically for NestJS applications, you can utilize the official package:

**Package:** `@multiversx/sdk-nestjs-cache`

This package exposes a **`CacheService`** which aggregates both **Redis** and **In-Memory** caching strategies into a single, unified service.

By injecting this service, you get the best of both worlds:
* **Local Memory:** For extremely fast access to frequently used data on the specific instance.
* **Redis:** For sharing the cache state across multiple instances and ensuring data persistence across restarts.

By implementing this layer, you transform your application from a simple proxy into a robust, scalable service capable of handling thousands of users without hitting public API limits.