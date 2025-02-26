use std::path::PathBuf;

use resvg::usvg;

#[derive(Default, Debug, Clone)]
pub struct FontOptions {
    skip_system_fonts: bool,
    font_files: Vec<PathBuf>,
    font_dirs: Vec<PathBuf>,
    serif_family: Option<String>,
    sans_serif_family: Option<String>,
    cursive_family: Option<String>,
    fantasy_family: Option<String>,
    monospace_family: Option<String>,
}

pub fn load_fonts(args: &FontOptions) -> anyhow::Result<usvg::Options> {
    let mut usvg = usvg::Options {
        font_family: args.sans_serif_family.clone().unwrap_or("Arial".into()),
        ..Default::default()
    };
    let fontdb = usvg.fontdb_mut();

    if !args.skip_system_fonts {
        fontdb.load_system_fonts();
    }

    for path in &args.font_files {
        if let Err(e) = fontdb.load_font_file(path) {
            log::warn!("Failed to load '{}' cause {}.", path.display(), e);
        }
    }

    for path in &args.font_dirs {
        fontdb.load_fonts_dir(path);
    }

    fontdb.set_serif_family(args.serif_family.as_deref().unwrap_or("Times New Roman"));
    fontdb.set_sans_serif_family(args.sans_serif_family.as_deref().unwrap_or("Arial"));
    fontdb.set_cursive_family(args.cursive_family.as_deref().unwrap_or("Comic Sans MS"));
    fontdb.set_fantasy_family(args.fantasy_family.as_deref().unwrap_or("Impact"));
    fontdb.set_monospace_family(args.monospace_family.as_deref().unwrap_or("Courier New"));

    Ok(usvg)
}
