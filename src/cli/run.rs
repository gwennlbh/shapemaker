use cargo::{
    GlobalContext,
    core::{
        Workspace,
        compiler::{BuildConfig, UserIntent},
        resolver::CliFeatures,
    },
    ops::{self, CompileFilter, Packages},
};
use std::path::Path;

pub fn run_project(package_path: &Path) -> anyhow::Result<()> {
    let cargoctx = GlobalContext::default()?;
    let workspace = Workspace::new(&package_path.join("Cargo.toml"), &cargoctx)?;

    ops::run(
        &workspace,
        &ops::CompileOptions {
            build_config: BuildConfig::new(
                &cargoctx,
                None,
                false,
                &[],
                UserIntent::Build,
            )?,
            cli_features: CliFeatures::new_all(false),
            spec: Packages::Default,
            filter: CompileFilter::new_all_targets(),
            target_rustdoc_args: None,
            target_rustc_args: None,
            target_rustc_crate_types: None,
            rustdoc_document_private_items: false,
            honor_rust_version: None,
        },
        &[],
    )?;
    Ok(())
}
