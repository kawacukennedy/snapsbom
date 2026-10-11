# SnapSBOM

> SBOMs in a snap. No cloud. No friction.

SnapSBOM generates deterministic [SBOMs](https://www.cisa.gov/resources-tools/resources/software-bill-materials-sbom) (Software Bill of Materials) in milliseconds. It runs entirely locally in your CLI or browser with zero network calls, zero telemetry, and byte-for-byte reproducible output.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![WASM Ready](https://img.shields.io/badge/WASM-ready-654FF0.svg)](https://webassembly.org/)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](./CONTRIBUTING.md)

## ✨ Features

- **Lightning fast**: Generates SBOMs in milliseconds, even for large projects
- **Truly local-first**: Zero network calls, zero telemetry, zero tracking. Works fully offline and in air-gapped environments.
- **Deterministic by default**: Same input always produces byte-for-byte identical output. Critical for verification and compliance.
- **Runs anywhere**: Identical behavior in CLI (native), Node.js (npx), and browser (WASM). No Docker required.
- **Standards compliant**: Outputs [SPDX 2.3](https://spdx.github.io/spdx-spec/v2.3/) and [CycloneDX 1.6](https://cyclonedx.org/specification/overview/) JSON.
- **Minimal & focused**: Analyzes lockfiles for accuracy with no heavy filesystem scanning. Fast, simple, and reliable.

## 🚀 Quick Start

### CLI (recommended)

```bash
# Run with npx (no install needed)
npx snapsbom .

# Or install globally
npm install -g snapsbom
snapsbom .
```

### Browser

Try it instantly in your browser with zero upload: [snapsbom.dev](https://snapsbom.dev)

Drag & drop your `package-lock.json`, `pnpm-lock.yaml`, `yarn.lock`, `go.mod`/`go.sum`, `poetry.lock`, or `pyproject.toml` - SBOM generates entirely in your browser.

### Output formats

```bash
# SPDX 2.3 (default)
snapsbom . --format spdx

# CycloneDX 1.6
snapsbom . --format cyclonedx

# Write to file
snapsbom . > sbom.spdx.json
```

## 🎯 Why Determinism Matters

Most SBOM tools produce slightly different output each run (different ordering, formatting). SnapSBOM guarantees byte-for-byte identical output for the same input across different machines, OSes, and runtimes. This is essential for:
- **Supply chain verification**: Reproducible builds and artifact attestation
- **Compliance**: Auditable, diffable SBOMs for regulations like the [EU Cyber Resilience Act (CRA)](https://digital-strategy.ec.europa.eu/en/library/cyber-resilience-act)
- **Trust**: Content hashes match regardless of where generation happens

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## 📄 License

MIT License - see [LICENSE](./LICENSE) for details.

## ⭐ Star History

If SnapSBOM helps you, please consider starring the repo to help others discover it!
