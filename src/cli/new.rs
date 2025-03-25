use anyhow::anyhow;
use cargo::{
    core::{dependency::DepKind, EitherManifest, Package, SourceId, Workspace},
    ops::{
        self,
        cargo_add::{self, AddOptions, DepOp},
        NewOptions, VersionControl,
    },
    util::{
        context::GlobalContext, toml::read_manifest, toml_mut::manifest::DepTable,
    },
};
use std::{env, fs, path::Path};

use crate::cli::run;

pub fn new_project(name: String) -> anyhow::Result<()> {
    let cargoctx = GlobalContext::default()?;
    let package_path = Path::new(&env::current_dir()?).join(&name);
    println!("Creating project at {:?}", package_path);

    // Create a bin crate with the name of the directory
    ops::new(
        &NewOptions {
            version_control: Some(VersionControl::NoVcs),
            kind: ops::NewProjectKind::Bin,
            auto_detect_kind: false,
            path: package_path.clone(),
            name: None,
            edition: None,
            registry: None,
        },
        &cargoctx,
    )?;

    println!("Reading manifest");

    let manifest = read_manifest(
        &package_path.clone().join("Cargo.toml"),
        SourceId::crates_io(&cargoctx)?,
        &cargoctx,
    )?;

    let manifest = match manifest {
            EitherManifest::Real(manifest) => manifest,
            EitherManifest::Virtual(_) => {
                return Err(anyhow!("Virtual manifests not supported, run the command outside of a workspace, or create your project manually with cargo new <name> && cd <name> && cargo add shapemaker && cargo add rand"))
            }
        };

    println!("Adding dependencies to Cargo.toml");

    let workspace = Workspace::new(&package_path.join("Cargo.toml"), &cargoctx)?;
    // Add deps
    cargo_add::add(
        &workspace,
        &AddOptions {
            dry_run: false,
            honor_rust_version: None,
            gctx: &cargoctx,
            spec: &Package::new(manifest, &package_path.join("Cargo.toml")),
            dependencies: vec![
                DepOp {
                    crate_spec: Some("shapemaker".to_string()),
                    rename: None,
                    features: None,
                    default_features: Some(true),
                    optional: Some(false),
                    public: None,
                    registry: None,
                    path: None,
                    base: None,
                    git: Some(
                        "https://github.com/gwennlbh/shapemaker".to_string(),
                    ),
                    branch: None,
                    rev: None,
                    tag: match env::var("CARGO_PKG_VERSION") {
                        Ok(version) => Some(version),
                        Err(_) => None,
                    },
                },
                DepOp {
                    crate_spec: Some("rand".to_string()),
                    rename: None,
                    features: None,
                    default_features: Some(true),
                    optional: Some(false),
                    public: None,
                    registry: None,
                    path: None,
                    base: None,
                    git: None,
                    branch: None,
                    rev: None,
                    tag: None,
                },
            ],
            section: DepTable::new().set_kind(DepKind::Normal),
        },
    )?;

    println!("Writing main.rs");

    // Write template main.rs
    fs::write(
        package_path.join("src/main.rs"),
        format!(
            "
use shapemaker::*;
use rand;

pub fn main() {{
    let mut canvas = Canvas::new(vec![]);

    // Make your canvas beautiful <3

    canvas.render_to_png(\"{}.png\", 2000).unwrap();
}}",
            name
        )
        .trim(),
    )?;


    run::run_project(&package_path)?;

    std::env::set_current_dir(&package_path)?;

    std::process::Command::new(
        std::env::var("SHAPEMAKER_EDITOR").unwrap_or_else(|_| {
            std::env::var("EDITOR").unwrap_or_else(|_| "code".to_string())
        }),
    )
    .arg(".")
    .spawn()?;

    return Ok(());
}
