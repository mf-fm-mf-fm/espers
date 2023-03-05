use espers::plugin::Plugin;
use espers::records::{Group, Record};

use std::fs::File;

use anyhow::Error;
use clap::Parser;

/// Dump contents of *.es[mp] files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to plugin file
    path: String,
}

pub fn dump(group: &Group, indent: usize) {
    println!("{:indent$}{}", "", group);

    for record in &group.records {
        match record {
            Record::Group(g) => dump(g, indent + 1),
            record => {
                let i = indent + 1;
                println!("{:i$}{}", "", record)
            }
        }
    }
}

pub fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut f = File::open(&args.path)?;

    let plugin = Plugin::parse(&mut f)?;

    for record in plugin.records {
        if let Record::Group(g) = record {
            dump(&g, 0);
        }
    }

    Ok(())
}
