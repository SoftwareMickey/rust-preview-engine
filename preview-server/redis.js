const Redis = require("ioredis");

const REDIS_URL = process.env.REDIS_URL;

// * Base Redis client
const redis = new Redis(REDIS_URL, {
  lazyConnect: true,
  maxRetriesPerRequest: null,
});

// * Separate connections for pub/sub (required by Redis protocol)
const redisPub = new Redis(REDIS_URL, {
  lazyConnect: true,
  maxRetriesPerRequest: null,
});

const redisSub = new Redis(REDIS_URL, {
  lazyConnect: true,
  maxRetriesPerRequest: null,
});

redis.on("connect", () => {
  console.log("REDIS CONNECTION ESTABLISHED!");
});

redis.on("error", (err) => {
  console.log("Redis connection error", err);
});

redisPub.on("connect", () => {
  console.log("REDIS PUBLISHER CONNECTION ESTABLISHED!");
});

redisPub.on("error", (err) => {
  console.log("redisPub connection error", err);
});

redisSub.on("connect", () => {
  console.log("REDIS SUBSCRIBER CONNECTION ESTABLISHED!");
});

redisSub.on("error", (err) => {
  console.log("redisSub connection error", err);
});

module.exports = { redis, redisPub, redisSub };
