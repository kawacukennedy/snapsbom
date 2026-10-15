# SnapSBOM: Deterministic SBOMs in a Snap

## The Problem: SBOM Tools Are Too Heavy

If you've ever tried to generate a Software Bill of Materials (SBOM) for a project, you've probably run into one of these issues:

- **Bloated tooling** - Tools require Docker, complex configurations, or cloud accounts just to generate a simple SBOM.
- **Inconsistent outputs** - Run the same tool twice on the same codebase and you get slightly different JSON. That's terrible for verification.
- **Slow feedback loops** - You shouldn't need to wait seconds (or minutes) for something this fundamental.
- **Overkill for most projects** - If all you need is an SBOM from your lockfiles, why download a 100MB binary?

With regulations like the [EU Cyber Resilience Act (CRA)](https://digital-strategy.ec.europa.eu/en/library/cyber-resilience-act) pushing SBOM adoption, developers need something that just works without the enterprise bloat.

## The Solution: SnapSBOM

[SnapSBOM](https://github.com/kawacukennedy/snapsbom) is a deterministic SBOM generator that runs instantly, locally, and produces identical output every single time.

```bash
npx snapsbom .
```

That's it. In milliseconds, you get a clean, standards-compliant SBOM.

![SnapSBOM Demo](https://raw.githubusercontent.com/kawacukennedy/snapsbom/main/docs/assets/demo.gif)

## Why It Stands Out

- **Deterministic by design** – Same input → byte-for-byte identical output. This is critical for reproducible builds, audits, and supply chain verification.
- **Truly local-first** – Zero network calls. Zero telemetry. Works offline and in air-gapped environments. Your dependency data never leaves your machine.
- **Runs anywhere** – CLI, `npx`, and browser (WASM) all share the same Rust core, so results are identical everywhere.
- **Standards compliant** – Outputs both [SPDX 2.3](https://spdx.github.io/spdx-spec/v2.3/) and [CycloneDX 1.6](https://cyclonedx.org/specification/overview/).
- **Lightning fast** – Parses lockfiles directly with no heavy filesystem scanning or container runtime.
- **Minimal by choice** – Focused on doing one thing exceptionally well: generating correct, deterministic SBOMs.

## Why Determinism Matters

Most SBOM tools serialize in arbitrary order. SnapSBOM canonicalizes everything (keys, arrays, components) so the hash of your SBOM is stable. This means:

- **Easier CI integration** – You can diff, cache, and verify SBOM artifacts with confidence.
- **Better supply chain trust** – Reproducible SBOMs make attestation and verification actually meaningful.
- **Compliance-ready** – Auditors can rely on consistent, diffable artifacts.

## Get Started Today

**With npx (fastest):**

```bash
npx snapsbom .
```

**Install globally:**

```bash
npm install -g snapsbom
snapsbom .
```

**Or download a binary from [GitHub Releases](https://github.com/kawacukennedy/snapsbom/releases).**

## Open Source & Built for Developers

SnapSBOM is [MIT licensed](https://github.com/kawacukennedy/snapsbom/blob/main/LICENSE) and built in Rust with a shared core for CLI and WASM. It's designed to be simple, fast, and reliable — the kind of tool you'd actually keep in your workflow.

- **GitHub:** [github.com/kawacukennedy/snapsbom](https://github.com/kawacukennedy/snapsbom)
- **npm:** [npmjs.com/package/snapsbom](https://www.npmjs.com/package/snapsbom)
- **Live Demo:** [kawacukennedy.github.io/snapsbom](https://kawacukennedy.github.io/snapsbom/)
- **Issues/Feedback:** [Open an issue](https://github.com/kawacukennedy/snapsbom/issues)

If SnapSBOM saves you time or solves this exact pain point, I'd love your feedback. Star the repo if it's useful, or file an issue if you'd like to see support for more ecosystems!

*SBOMs shouldn't be heavy. They should just work.*
