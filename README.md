# SnapSBOM

<div align="center">

[![npm](https://img.shields.io/npm/v/snapsbom.svg)](https://www.npmjs.com/package/snapsbom)
[![Crates.io](https://img.shields.io/crates/v/snapsbom.svg)](https://crates.io/crates/snapsbom)
[![GitHub Release](https://img.shields.io/github/release/kawacukennedy/snapsbom.svg)](https://github.com/kawacukennedy/snapsbom/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/kawacukennedy/snapsbom/ci.yml?branch=main)](https://github.com/kawacukennedy/snapsbom/actions)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](./CONTRIBUTING.md)

**Deterministic SBOM generation that runs anywhere.**

[Demo](https://kawacukennedy.github.io/snapsbom/) | [Docs](https://github.com/kawacukennedy/snapsbom#installation) | [npm](https://www.npmjs.com/package/snapsbom) | [Releases](https://github.com/kawacukennedy/snapsbom/releases)

</div>

SnapSBOM generates deterministic [SBOMs](https://www.cisa.gov/resources-tools/resources/software-bill-materials-sbom) (Software Bill of Materials) in milliseconds. It runs entirely locally in CLI or browser with zero network calls, zero telemetry, and byte-for-byte reproducible output.

## Features

- **Deterministic by design**: Same input always produces byte-for-byte identical output for verification and compliance.
- **Local-first**: Zero network calls, zero telemetry. Works offline and in air-gapped environments.
- **Runs anywhere**: CLI, npx, and browser share the same Rust core for identical results.
- **Standards compliant**: Outputs [SPDX 2.3](https://spdx.github.io/spdx-spec/v2.3/) and [CycloneDX 1.6](https://cyclonedx.org/specification/overview/).
- **Fast & minimal**: Analyzes lockfiles directly. No heavy scanning, no Docker required.

## Installation

### Use with npx (no install needed)

```bash
npx snapsbom .
```

### npm global install

```bash
npm install -g snapsbom
snapsbom .
```

### Download binary

Download the latest binary for your platform from [GitHub Releases](https://github.com/kawacukennedy/snapsbom/releases).

### Install from source

```bash
cargo install --git https://github.com/kawacukennedy/snapsbom
snapsbom .
```

## Usage

```bash
# Generate SPDX (default)
snapsbom .

# Generate CycloneDX
snapsbom . --format cyclonedx

# Specify lockfile or directory
snapsbom package-lock.json
snapsbom . --output sbom.spdx.json
```

## Why determinism matters

Deterministic output enables reproducible builds, diffable SBOMs, and trustworthy supply chain verification - critical for compliance like the [EU Cyber Resilience Act (CRA)](https://digital-strategy.ec.europa.eu/en/library/cyber-resilience-act).

## Contributing

Contributions welcome. See [CONTRIBUTING.md](./CONTRIBUTING.md).

## License

MIT. See [LICENSE](./LICENSE).

## Acknowledgments

Built with Rust, focused on simplicity, correctness, and developer experience.
