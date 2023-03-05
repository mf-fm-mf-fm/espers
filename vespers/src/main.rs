mod app;

use app::VespersApp;

use espers::plugin::Plugin;
use espers::string_table::StringTables;

use std::fs::File;

use clap::Parser;
use iced::{Application, Settings};

/// Dump contents of *.es[mp] files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to plugin file
    path: String,

    /// Which language to load localized strings for
    #[clap(long, short, default_value = "English")]
    language: String,
}

pub fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let mut plugin = Plugin::parse(&mut File::open(&args.path)?)?;
    let strings = StringTables::load(args.path.as_ref(), args.language.as_ref())?;

    plugin.localize(&strings);

    VespersApp::run(Settings::with_flags((plugin, args)))?;

    Ok(())
}
