use clap::{Parser, ValueEnum};
use snapsbom::{cyclonedx, det, npm, spdx, Format};
use std::path::{Path, PathBuf};
use std::time::Instant;

const EXAMPLES: &str = "\
Examples:
  snapsbom .                          SPDX 2.3 to stdout
  snapsbom . --format cyclonedx       CycloneDX 1.6 to stdout
  snapsbom . --output sbom.spdx.json  write to a file
  snapsbom path/to/package-lock.json  analyse a specific lockfile

Reads npm lockfiles (v1 / v2 / v3). No network. Byte-for-byte reproducible.";

#[derive(Parser, Debug)]
#[command(
    name = "snapsbom",
    version,
    about = "Deterministic SBOM generator",
    long_about = "Deterministic SBOM generator\n\n\
                  Reads an npm package-lock.json and writes SPDX 2.3 or CycloneDX 1.6 JSON.\n\
                  Everything runs locally: no network, no telemetry, and the same input\n\
                  always produces byte-for-byte identical output.",
    after_help = EXAMPLES,
    after_long_help = EXAMPLES
)]
struct Args {
    #[arg(
        value_name = "PATH",
        help = "Lockfile or a directory containing package-lock.json"
    )]
    path: PathBuf,

    #[arg(
        long,
        value_enum,
        default_value = "spdx",
        value_name = "FORMAT",
        help = "Output format"
    )]
    format: Fmt,

    #[arg(long, value_name = "FILE", help = "Write to a file instead of stdout")]
    output: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Fmt {
    Spdx,
    Cyclonedx,
}

impl Fmt {
    /// Human label used in the success line and in generated documents.
    fn label(self) -> &'static str {
        match self {
            Fmt::Spdx => "SPDX 2.3",
            Fmt::Cyclonedx => "CycloneDX 1.6",
        }
    }
}

impl From<Fmt> for Format {
    fn from(f: Fmt) -> Self {
        match f {
            Fmt::Spdx => Format::Spdx,
            Fmt::Cyclonedx => Format::Cyclonedx,
        }
    }
}

/// Turn the user's argument into a concrete lockfile path.
///
/// Failing here with a message that says what to do next beats letting
/// `read` surface a bare "Is a directory (os error 21)".
fn resolve_lockfile(input: &Path) -> anyhow::Result<PathBuf> {
    if input.is_dir() {
        let candidate = input.join("package-lock.json");
        if candidate.exists() {
            return Ok(candidate);
        }
        return Err(anyhow::anyhow!(
            "no package-lock.json found in {}\n\n  SnapSBOM reads npm lockfiles, so either:\n    * pass a directory that contains package-lock.json\n    * or pass the file directly: snapsbom path/to/package-lock.json",
            input.display()
        ));
    }
    if !input.exists() {
        return Err(anyhow::anyhow!(
            "{} does not exist\n\n  Check the path, or run snapsbom . from your project root.",
            input.display()
        ));
    }
    Ok(input.to_path_buf())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let p = resolve_lockfile(&args.path)?;
    let started = Instant::now();

    let comps = npm::parse_package_lock(&p)?;
    if comps.is_empty() {
        // Never fail silently on a zero-package SBOM - that is exactly the
        // mistake a CI integration would not notice.
        eprintln!(
            "warning: {} yielded 0 packages - the SBOM will be empty",
            p.display()
        );
    }

    let v = match args.format.into() {
        Format::Spdx => spdx::components_to_spdx(&comps),
        Format::Cyclonedx => cyclonedx::components_to_cyclonedx(&comps),
    };
    let out = det::to_deterministic_json(&v)?;

    match args.output {
        Some(ref f) => {
            std::fs::write(f, &out)?;
            // stdout stays clean for pipes; progress belongs on stderr.
            eprintln!(
                "✔ {} · {} components · {} · {} ms",
                f.display(),
                comps.len(),
                args.format.label(),
                started.elapsed().as_millis()
            );
        }
        None => print!("{}", out),
    }
    Ok(())
}
