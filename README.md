<div align="center">
  <img src="docs/assets/banner.png" alt="SnapSBOM — deterministic SBOMs in milliseconds" width="900" />
</div>

<br/>

<div align="center">

[![npm version](https://img.shields.io/npm/v/snapsbom?color=41e3ff&label=npm)](https://www.npmjs.com/package/snapsbom)
[![npm downloads](https://img.shields.io/npm/dm/snapsbom?color=5b7cff)](https://www.npmjs.com/package/snapsbom)
[![CI](https://img.shields.io/github/actions/workflow/status/kawacukennedy/snapsbom/ci.yml?branch=main&label=CI)](https://github.com/kawacukennedy/snapsbom/actions)
[![License: MIT](https://img.shields.io/badge/license-MIT-39e1a3.svg)](./LICENSE)
[![PRs welcome](https://img.shields.io/badge/PRs-welcome-9d7bff.svg)](./CONTRIBUTING.md)
[![Rust](https://img.shields.io/badge/built%20with-Rust-f74c00?logo=rust&logoColor=white)](https://www.rust-lang.org/)

</div>

<div align="center">
  <h3>Deterministic SBOMs in milliseconds.</h3>
  <p>
    SnapSBOM turns your lockfiles into <b>SPDX&nbsp;2.3</b> and <b>CycloneDX&nbsp;1.6</b> Software Bills of Materials —<br/>
    entirely on your machine, with <b>zero network calls</b> and <b>byte-for-byte reproducible</b> output.
  </p>
  <p>
    <a href="https://kawacukennedy.github.io/snapsbom/"><b>Live demo</b></a> &nbsp;·&nbsp;
    <a href="#quick-start">Quick start</a> &nbsp;·&nbsp;
    <a href="#determinism">Why determinism</a> &nbsp;·&nbsp;
    <a href="#interoperability">Interop</a> &nbsp;·&nbsp;
    <a href="#cli-reference">CLI reference</a> &nbsp;·&nbsp;
    <a href="https://www.npmjs.com/package/snapsbom">npm</a>
  </p>
</div>

<div align="center">
  <img src="docs/assets/demo.gif" alt="Terminal demo: npx snapsbom . generating an SPDX SBOM" width="900" />
</div>

<br/>

---

## What is SnapSBOM?

[SBOMs](https://www.cisa.gov/sbom) are becoming mandatory — the **EU Cyber Resilience Act**, US executive orders, and enterprise procurement all expect them. Yet most SBOM tools are heavyweight scanners that require a build, a container, or a cloud upload, and that emit *different output every run*.

SnapSBOM takes the opposite approach. It **reads your lockfile directly** and emits a standards-compliant SBOM in a few milliseconds, using a canonical, sorted, timestamp-pinned representation so the same input always produces **exactly the same bytes**.

```console
$ npx snapsbom .
✔ Wrote SPDX 2.3 · 7 components · 3 ms
```

> **Same lockfile in, same bytes out — on any machine, any day.** That is what makes an SBOM *verifiable* instead of merely *present*.

---

<a id="quick-start"></a>
## ⚡ Quick start

No install required:

```bash
npx snapsbom .
```

Prefer a persistent install?

```bash
# Global npm install
npm install -g snapsbom
snapsbom .

# …or build the CLI from source
cargo install --path .   # after cloning this repo
```

Point it at a directory (it finds `package-lock.json` automatically) or straight at a lockfile:

```bash
snapsbom ./my-app
snapsbom ./my-app/package-lock.json
```

---

## ✨ Features

| | |
|---|---|
| **🎯 Deterministic by design** | Fixed timestamps, sorted keys, canonical package IDs and ordering. Identical input → identical bytes, forever. |
| **🔒 Truly local-first** | Zero network calls. Zero telemetry. Runs offline, in CI, and in air-gapped environments. |
| **⚙️ One core, two runtimes** | The same Rust core powers the CLI and the browser (WASM) playground — so both agree exactly. |
| **📦 Standards compliant** | Emits [SPDX 2.3](https://spdx.github.io/spdx-spec/v2.3/) and [CycloneDX 1.6](https://cyclonedx.org/specification/overview/) JSON. |
| **🚀 Fast & dependency-light** | Parses lockfiles directly — no source scan, no container, no Docker. Milliseconds, not minutes. |
| **🧩 Tiny surface** | A single static binary (or a `npx` one-liner). Nothing to configure to get a valid SBOM. |

---

<a id="determinism"></a>
## 🔬 Why determinism matters

A normal SBOM generator stamps the current time, walks the filesystem in an arbitrary order, and assigns random IDs. Run it twice and you get two different files — so you cannot tell whether a change in the diff is a **real dependency change** or just noise. That makes review, diffing, and attestation painful.

SnapSBOM produces a **canonical** document instead:

- 🕒 `created` is pinned to a fixed epoch (`2026-01-01T00:00:00Z`), not “now”.
- 🔢 Packages are **deduplicated by purl and sorted**, so `SPDXRef-Package-1…N` are stable and predictable.
- 🔑 Keys are emitted in a stable order, so line-level diffs are meaningful.
- 🧾 The `documentNamespace` is stable, so the document identity doesn’t churn.

The result: you can commit an SBOM next to the lockfile and **`git diff` it like any other file**. Verification becomes a string comparison.

<div align="center">
  <img src="docs/assets/determinism.gif" alt="Running SnapSBOM twice produces identical SHA-256 output" width="820" />
</div>

```bash
$ snapsbom examples/demo | sha256sum
ff8584cdc321a348c94c9d8e139149556ab8b895d8b83da68e41013fe38c7689  -

$ snapsbom examples/demo | sha256sum   # …and again
ff8584cdc321a348c94c9d8e139149556ab8b895d8b83da68e41013fe38c7689  -
```

---

## 📤 Output formats

### SPDX 2.3 (default)

```bash
snapsbom . --format spdx
```

<details>
<summary>See sample output</summary>

```json
{
  "SPDXID": "SPDXRef-DOCUMENT",
  "spdxVersion": "SPDX-2.3",
  "dataLicense": "CC0-1.0",
  "name": "SnapSBOM SBOM",
  "documentNamespace": "https://spdx.org/spdxdocs/SPDXRef-DOCUMENT-STATIC",
  "creationInfo": {
    "created": "2026-01-01T00:00:00Z",
    "creators": ["Tool: SnapSBOM"]
  },
  "packages": [
    {
      "SPDXID": "SPDXRef-Package-1",
      "name": "@types/node",
      "versionInfo": "22.10.2",
      "downloadLocation": "NOASSERTION",
      "filesAnalyzed": false,
      "licenseConcluded": "NOASSERTION",
      "licenseDeclared": "NOASSERTION",
      "copyrightText": "NOASSERTION",
      "supplier": "NOASSERTION",
      "externalRefs": [
        {
          "referenceCategory": "PACKAGE-MANAGER",
          "referenceType": "purl",
          "referenceLocator": "pkg:npm/%40types/node@22.10.2"
        }
      ]
    }
  ],
  "relationships": [
    {
      "spdxElementId": "SPDXRef-DOCUMENT",
      "relationshipType": "DESCRIBES",
      "relatedSpdxElement": "SPDXRef-Package-1"
    }
  ]
}
```

</details>

### CycloneDX 1.6

```bash
snapsbom . --format cyclonedx
```

<details>
<summary>See sample output</summary>

```json
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.6",
  "version": 1,
  "components": [
    {
      "type": "library",
      "name": "@types/node",
      "version": "22.10.2",
      "purl": "pkg:npm/%40types/node@22.10.2",
      "supplier": { "name": "NOASSERTION" },
      "licenses": []
    }
  ]
}
```

</details>

---

<a id="cli-reference"></a>
## 🧰 CLI reference

```text
Deterministic SBOM generator

Usage: snapsbom [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to lockfile (e.g. package-lock.json) or directory

Options:
      --format <FORMAT>  [default: spdx] [possible values: spdx, cyclonedx]
      --output <OUTPUT>  Output file (stdout if not specified)
  -h, --help             Print help
  -V, --version          Print version
```

| Command | What it does |
|---|---|
| `snapsbom .` | Generate SPDX from `package-lock.json` in the current directory. |
| `snapsbom . --format cyclonedx` | Generate a CycloneDX 1.6 document. |
| `snapsbom . --output sbom.spdx.json` | Write to a file instead of stdout. |
| `snapsbom path/to/package-lock.json` | Analyze a specific lockfile. |

---

## 🗂️ Supported inputs

| Ecosystem | Lockfile | Status |
|---|---|---|
| **npm** | `package-lock.json` (v1 / v2 / v3) | ✅ Supported |
| pnpm | `pnpm-lock.yaml` | 🛣️ Roadmap |
| Yarn | `yarn.lock` | 🛣️ Roadmap |
| Python | `poetry.lock`, `requirements.txt` | 🛣️ Roadmap |
| Cargo | `Cargo.lock` | 🛣️ Roadmap |

SnapSBOM is intentionally **honest about scope**: it currently guarantees correctness for npm lockfiles, and grows one ecosystem at a time. PRs for new parsers are very welcome — the parser plug-in point lives in [`src/npm.rs`](./src/npm.rs).

---

## 🌐 Interoperability

A deterministic SBOM is only useful if the rest of your toolchain can read it. SnapSBOM's output is verified against the **official specifications** and against **independent implementations** — never just against itself.

Everything below was measured on one fixture: a real `npm install express react` project (`package-lock.json` v3, 67 entries → **66 resolved packages**).

### Schema validation

| Output | Validator | Result |
|---|---|---|
| CycloneDX 1.6 | `ajv` vs the official [`bom-1.6.schema.json`](https://github.com/CycloneDX/specification/blob/master/schema/bom-1.6.schema.json) (with its `spdx` + `jsf` refs) | ✅ valid |
| SPDX 2.3 | `ajv` vs the official [`spdx-schema.json`](https://github.com/spdx/spdx-spec/blob/v2.3/schemas/spdx-schema.json) | ✅ valid |

### Ingestion by other tools

| Tool | Command | Result |
|---|---|---|
| Syft 1.55.0 | `syft convert app.cdx.json -o spdx-json=out.spdx.json` | ✅ ingested, re-emitted as SPDX 2.3 |
| Syft 1.55.0 | `syft convert app.spdx.json -o cyclonedx-json=out.cdx.json` | ✅ ingested, all **66** components round-tripped |
| Trivy 0.76.0 | `trivy sbom app.cdx.json` | ✅ detected as `cyclonedx-json`, **66/66 packages** ingested, scan completes |
| Trivy 0.76.0 | `trivy sbom app.spdx.json` | ✅ detected as `spdx-json`, **66/66 packages** ingested, scan completes |

Three independent implementations — **Syft**, **Trivy**, and **cyclonedx-npm** — all converge on the same 66 packages.

### How it compares

Same lockfile, same machine, every number measured locally:

| | SnapSBOM 0.2.2 | Syft 1.55.0 | [`@cyclonedx/cyclonedx-npm`](https://github.com/CycloneDX/cyclonedx-npm) 6.0.1 |
|---|---|---|---|
| Reads | `package-lock.json` (v1–v3) only | files, dirs, container images, archives | `package-lock.json` + installed `node_modules` |
| Emits | SPDX 2.3 **and** CycloneDX 1.6 | SPDX 2.2/2.3, CycloneDX, Syft JSON | CycloneDX only |
| Packages on this fixture | **66** | 68 (the 66 + the scanned project + the lockfile file itself) | **66** |
| Byte-identical on re-run | ✅ always | ❌ `metadata.timestamp` churns | ❌ by default; ✅ with `--output-reproducible`, which drops the timestamp *and* serial number |
| `created` / `timestamp` | pinned to `2026-01-01T00:00:00Z` | wall clock | wall clock (or removed entirely) |
| Works offline, no install | ✅ lockfile only | ✅ for directory scans | ❌ requires an installed `node_modules` (invokes `npm`) |

The strongest signal a format can offer is **two independent implementations agreeing**. SnapSBOM and `@cyclonedx/cyclonedx-npm` produce the *same 66 packages and byte-identical purls* on this fixture:

```bash
$ python3 - <<'EOF'
import json
ours  = {(c['name'], c['version'], c['purl']) for c in json.load(open('app.cdx.json'))['components']}
theirs= {(c['name'], c['version'], c['purl']) for c in json.load(open('cdxnpm.json'))['components']}
print('identical package sets:', ours == theirs)
print('purl differences:', len(ours ^ theirs))
EOF
identical package sets: True
purl differences: 0
```

### Reproduce every number above

```bash
# 0. the fixture
mkdir fixture && (cd fixture && npm init -y && npm install express react)

# 1. SnapSBOM output, both formats
snapsbom fixture --format cyclonedx --output app.cdx.json
snapsbom fixture --format spdx      --output app.spdx.json

# 2. schema validation against the official specs
#    (rewrite each schema's $id to its local filename first,
#     so ajv's -r can resolve the refs)
npx ajv-cli@5 validate -s bom-1.6.schema.json -r spdx.schema.json -r jsf-0.82.schema.json \
  -d app.cdx.json --spec=draft7 --strict=false --validate-formats=false
npx ajv-cli@5 validate -s spdx-schema.json \
  -d app.spdx.json --spec=draft7 --strict=false --validate-formats=false

# 3. ingestion by an independent implementation
syft convert app.cdx.json  -o spdx-json=from-cdx.spdx.json
syft convert app.spdx.json -o cyclonedx-json=from-spdx.cdx.json

# 4. determinism — run twice, diff must be empty
snapsbom fixture --format cyclonedx --output run1.json
snapsbom fixture --format cyclonedx --output run2.json
diff run1.json run2.json && echo "byte-identical"

# 5. cross-check against an independent implementation (compare with the
#    python snippet above — identical package sets, zero purl differences)
npx @cyclonedx/cyclonedx-npm -o cdxnpm.json --of JSON --sv 1.6 fixture/package.json

# 6. vulnerability scanning by an independent implementation
#    (trivy downloads its vulnerability DB on first run)
trivy sbom app.cdx.json
trivy sbom app.spdx.json
```

---

## 🏗️ How it works

```
        package-lock.json
               │
               ▼
      ┌───────────────────┐
      │  Parser (npm)     │  read lockfile → component set
      └───────────────────┘
               │
               ▼
      ┌───────────────────┐
      │  Canonicalise     │  dedupe by purl · sort · pin IDs
      └───────────────────┘
               │
               ▼
      ┌───────────────────┐
      │  Emitter          │  SPDX 2.3  |  CycloneDX 1.6
      └───────────────────┘
               │
               ▼
          SBOM (stdout / file)
```

One Rust crate (`snapsbom`) provides the parser, the deterministic canonicaliser, and both emitters. It's compiled twice:

- as a **CLI** (`src/main.rs`, via `clap`) — the binary you run with `npx` or `cargo`;
- as a **WASM module** (`src/wasm.rs`, via `wasm-bindgen`) — used by the [browser demo](https://kawacukennedy.github.io/snapsbom/), where everything runs client-side.

Because both share the same core, the browser and the terminal produce identical documents.

---

## 🔄 Use it in CI

Generate an SBOM as a build artifact — no secrets, no network, no external service:

```yaml
name: SBOM
on:
  push:
    branches: [main]

jobs:
  sbom:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 20
      - name: Generate SBOM
        run: npx --yes snapsbom . --format cyclonedx --output sbom.cdx.json
      - uses: actions/upload-artifact@v4
        with:
          name: sbom
          path: sbom.cdx.json
```

---

## 🤝 Contributing

Contributions are welcome — new lockfile parsers, emitter tweaks, docs, and tests especially.

- Read [`CONTRIBUTING.md`](./CONTRIBUTING.md) to get started.
- Run `cargo test` before opening a PR.
- See [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md) for community expectations.
- Found a security issue? Please follow [`SECURITY.md`](./SECURITY.md).

---

## 📄 License

MIT — see [`LICENSE`](./LICENSE).

<div align="center">
  <sub>Built with 🦀 Rust. No cloud. No telemetry. No nonsense.</sub>
</div>
