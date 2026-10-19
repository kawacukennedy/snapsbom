#!/usr/bin/env node
"use strict";

/*
 * SnapSBOM launcher.
 *
 * The real work lives in a small Rust binary. Rather than run a `postinstall`
 * script (which downloads code at install time and is blocked by
 * `--ignore-scripts`), we ship the binary as an optional, per-platform
 * dependency and simply exec the matching one here.
 *
 * This mirrors how esbuild / swc / rollup distribute native binaries.
 */

const { spawnSync } = require("child_process");
const fs = require("fs");
const path = require("path");

const platform = process.platform; // darwin | linux | win32
const arch = process.arch; // x64 | arm64

const exe = platform === "win32" ? "snapsbom.exe" : "snapsbom";
const pkg = `snapsbom-${platform}-${arch}`;

function resolveBinary() {
  // 1. The optional dependency installed for this platform.
  try {
    const pkgJson = require.resolve(`${pkg}/package.json`);
    const candidate = path.join(path.dirname(pkgJson), "bin", exe);
    if (fs.existsSync(candidate)) return candidate;
  } catch (_) {
    /* not installed — fall through */
  }

  // 2. A binary sitting next to this script (local dev / manual install).
  const local = path.join(__dirname, exe);
  if (fs.existsSync(local)) return local;

  return null;
}

const binary = resolveBinary();

if (!binary) {
  const lines = [
    `snapsbom: no prebuilt binary was found for ${platform}-${arch}.`,
    "",
    `This usually means the optional dependency "${pkg}" was skipped.`,
    "That happens with `npm install --no-optional`, or on an unsupported platform.",
    "",
    "Fix it with either:",
    `  • npm install ${pkg}`,
    "  • cargo install --git https://github.com/kawacukennedy/snapsbom",
    "",
  ];
  process.stderr.write(lines.join("\n"));
  process.exit(1);
}

const result = spawnSync(binary, process.argv.slice(2), { stdio: "inherit" });

if (result.error) {
  process.stderr.write(
    `snapsbom: failed to run ${binary}\n  ${result.error.message}\n`
  );
  process.exit(1);
}

process.exit(result.status === null ? 1 : result.status);
