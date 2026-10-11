use clap::{Parser, ValueEnum};
use snapsbom::{generate_from_path, Format};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "snapsbom", about = "Deterministic SBOM generator", version = "0.1.0")]
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

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let p = if args.path.is_dir() {
        let candidate = args.path.join("package-lock.json");
        if candidate.exists() { candidate } else { args.path.clone() }
    } else {
        args.path.clone()
    };
    let out = generate_from_path(&p, args.format.into())?;
    match args.output {
        Some(f) => std::fs::write(f, out)?,
        None => print!("{}", out),
    }
    Ok(())
}
