require("dotenv").config();
const { spawn } = require("child_process");
const http = require("http");
const path = require("path");
const fs = require("fs");

const { app } = require("./src/app/app");
const { redisSub, redisPub, redis } = require("./redis");
const { getAvailablePort } = require("./helpers");
const { HttpReverseProxySite } = require("./reverse_proxy");

const PORT = process.env.PORT || 8000;
const buildsRoot = process.env.PROJECT_ROOT || "/sclera_builds";
const NPM_PATH = process.env.NPM_PATH;
const PREVIEW_URL = process.env.PREVIEW_URL;

const server = http.createServer(app);

(async () => {
  await redis.connect();
})();

redisSub.subscribe("preview_jobs", (err) => {
  if (err) {
    console.log(`Failed to subscribe to preview jobs : ${err}`);
    return;
  }

  console.log("✅ SUBSCRIBED TO PREVIEW JOBS CHANNEL");
});

redisSub.on("message", async (channel, message) => {
  if (channel !== "preview_jobs") return;

  const job = JSON.parse(message);
  const { siteName } = job;
  const projectDir = path.join(buildsRoot, siteName);

  console.log(`📦 Received preview job for ${siteName} at ${projectDir}`);

  if (!fs.existsSync(projectDir)) {
    console.error(`❌ Project directory does not exist: ${projectDir}`);
    return;
  }

  console.log(`PROJECT EXIST ON : ${projectDir}`);

  let REDIS_KEY = `preview:6882534919e80c0f5ab40ebe-${siteName}`;

  // * CHECK IF THE PROCESS IS RUNNING ON REDIS ALREADY
  try {
    const json = await redis.get(REDIS_KEY);

    if (json) {
      try {
        const info = JSON.parse(json);
        console.log(
          `⚠️ Dev server already running for ${siteName} on port ${info.port}`
        );

        HttpReverseProxySite({ siteID : siteName, port: info.port })

        redisPub.publish(
          "preview-config",
          JSON.stringify({
            preview_path: `${PREVIEW_URL}:${info.port}/sites/6882534919e80c0f5ab40ebe/${siteName}/home`,
          })
        );

        return;
      } catch (parseErr) {
        console.error("Failed to parse Redis JSON:", parseErr);
      }
    }
  } catch (err) {
    console.error("Redis get failed:", err);
  }

  const port = await getAvailablePort();
  console.log(`🚀 Starting Vite dev server for ${siteName} on port ${port}`);

  // // * Optional: run install
//   const npmExec = process.platform === "win32" ? "npm.cmd" : "npm";

//   console.log("npmExec =", npmExec);
//   console.log("projectDir exists?", fs.existsSync(projectDir), projectDir);

  const command = spawn(NPM_PATH, ["install"], {
    cwd: projectDir,
    stdio: "inherit",
  });

  const child = spawn(
    NPM_PATH,
    ["run", "dev", "--", "--port", port, "--host", "0.0.0.0"],
    {
      cwd: projectDir,
      stdio: "inherit",
    }
  );

  HttpReverseProxySite({ siteID : siteName, port: port })

  redisPub.publish(
    "preview-config",
    JSON.stringify({
      preview_path: `${PREVIEW_URL}:${port}/sites/6882534919e80c0f5ab40ebe/${siteName}/home`,
    })
  );

  // Optionally save to Redis

  await redis.set(
    `preview:6882534919e80c0f5ab40ebe-${siteName}`,
    JSON.stringify({ port, pid: child.pid })
  );
  await redis.expire(`preview:6882534919e80c0f5ab40ebe-${siteName}`, 600); // 10 minutes
});

server.listen(PORT, () => {
  console.log(`PREVIEW SERVER IS LISTENING ON PORT ${PORT}`);
});
