#!/usr/bin/env node

const { spawn } = require("child_process");
const { getBinaryPath } = require("./index.js");
const fs = require("fs");

function runBinary() {
  try {
    const binaryPath = getBinaryPath();
    
    // Check if binary exists
    if (!fs.existsSync(binaryPath)) {
      console.error(`❌ Binary not found at: ${binaryPath}`);
      console.error("Try reinstalling the package:");
      console.error("  npm uninstall @pulseengine/timedate-mcp-server");
      console.error("  npm install @pulseengine/timedate-mcp-server");
      process.exit(1);
    }
    
    // Make sure binary is executable on Unix systems
    if (process.platform !== "win32") {
      try {
        fs.chmodSync(binaryPath, 0o755);
      } catch (err) {
        // Ignore chmod errors, might not have permissions
      }
    }
    
    // Spawn the binary with all arguments passed through
    const child = spawn(binaryPath, process.argv.slice(2), {
      stdio: "inherit",
      shell: false
    });
    
    // Handle child process events
    child.on("error", (err) => {
      console.error(`❌ Failed to start timedate-mcp-server: ${err.message}`);
      process.exit(1);
    });
    
    child.on("exit", (code, signal) => {
      if (signal) {
        process.exit(128 + (process.platform === "win32" ? 1 : require("os").constants.signals[signal] || 1));
      } else {
        process.exit(code || 0);
      }
    });
    
    // Handle process termination
    process.on("SIGINT", () => {
      child.kill("SIGINT");
    });
    
    process.on("SIGTERM", () => {
      child.kill("SIGTERM");
    });
    
  } catch (err) {
    console.error(`❌ Error: ${err.message}`);
    process.exit(1);
  }
}

runBinary();