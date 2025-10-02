#!/usr/bin/env node

const path = require("path");
const fs = require("fs");

function cleanup() {
  const binDir = path.join(__dirname, "bin");

  try {
    if (fs.existsSync(binDir)) {
      fs.rmSync(binDir, { recursive: true, force: true });
      console.log("🧹 Cleaned up downloaded binaries");
    }
  } catch (err) {
    console.warn(`⚠️ Warning: Could not clean up bin directory: ${err.message}`);
  }
}

cleanup();
