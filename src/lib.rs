pub mod model;
pub mod npm;
pub mod spdx;
pub mod cyclonedx;
pub mod det;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Spdx,
    Cyclonedx,
}

pub fn generate_from_path(path: &Path, format: Format) -> anyhow::Result<String> {
    generate_from_path_with_options(path, format, None)
}

pub fn generate_from_path_with_options(
    path: &Path,
    format: Format,
    serial_number: Option<String>,
) -> anyhow::Result<String> {
    let comps = npm::parse_package_lock(path)?;
    let v = match format {
        Format::Spdx => spdx::components_to_spdx(&comps),
        Format::Cyclonedx => cyclonedx::components_to_cyclonedx_opts(&comps, serial_number),
    };
    det::to_deterministic_json(&v)
}
