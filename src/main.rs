use clap::{Parser, ValueEnum};
use snapsbom::{cyclonedx, det, npm, spdx, Format};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "snapsbom", about = "Deterministic SBOM generator", version)]
struct Args {
    #[arg(help = "Path to lockfile (e.g. package-lock.json) or directory")]
    path: PathBuf,
    #[arg(long, value_enum, default_value = "spdx")]
    format: Fmt,
    #[arg(long, help = "Output file (stdout if not specified)")]
    output: Option<PathBuf>,
}

#[derive(Debug, Clone, ValueEnum)]
enum Fmt {
    Spdx,
    Cyclonedx,
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
            "{} does not exist",
            input.display()
        ));
    }
    Ok(input.to_path_buf())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let p = resolve_lockfile(&args.path)?;

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
        Some(f) => std::fs::write(f, out)?,
        None => print!("{}", out),
    }
    Ok(())
}
