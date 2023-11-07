//! Library for reading `*.es[mp]` files
//! Example usage:
//!
//! ```
//! use espers::Game;
//! let game = Game::load(&vec!["assets/"], "English").unwrap();
//! ````

/// Utility functions
pub mod common;

/// Errors related to processing plugin files
pub mod error;

/// Field structs contained in a [records::Record]
pub mod fields;

/// Overarching struct that can load multiple plugin files
pub mod game;

/// Loads an individual `*.es[mp]` file, contained in a [game::Game]
pub mod plugin;

/// Record structs contained in a [plugin::Plugin]
pub mod records;

/// Structs for loading string table files
pub mod string_table;

pub use game::Game;
