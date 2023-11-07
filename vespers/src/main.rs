mod app;
mod widgets;

use app::VespersApp;

use clap::Parser;
use espers::game::Game;
use iced::{Application, Settings};

/// Show contents of `*.es[mp]` files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Paths to plugin files
    paths: Vec<String>,

    /// Which language to load localized strings for
    #[clap(long, short, default_value = "English")]
    language: String,
}

pub fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let paths: Vec<_> = args.paths.iter().map(AsRef::as_ref).collect();
    let game = Game::load(&paths, args.language.as_ref())?;

    VespersApp::run(Settings::with_flags((game, args)))?;

    Ok(())
}
