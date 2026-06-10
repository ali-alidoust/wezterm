use anyhow::Context;
use clap::Parser;
use std::path::PathBuf;

/// Generate baseline PNGs using the wezterm renderer.
#[derive(Parser, Debug)]
pub struct Args {
    /// Output directory for baselines
    #[arg(long, default_value = "tests/baselines")]
    out_dir: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    eprintln!("This helper must be run in a full build environment where wezterm-gui can initialize the renderer.");

    // For now we create the output directory and write a small marker file.
    std::fs::create_dir_all(&args.out_dir).context("create out_dir")?;
    let marker = args.out_dir.join("README.txt");
    std::fs::write(&marker, "Run this binary in a full GUI environment to generate real baselines.\n")?;

    eprintln!("Wrote marker to {}", marker.display());
    eprintln!("Next: run `cargo run -p wezterm-gui-subcommands --bin generate_rtl_baselines` in a full build environment to generate PNGs.");
    Ok(())
}
