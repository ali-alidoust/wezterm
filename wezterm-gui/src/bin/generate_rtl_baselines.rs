//! Baseline generator for mirror RTL runs.
//!
//! This binary is intended to be run from the wezterm-gui crate where it
//! can access renderer internals. It currently generates placeholder PNGs
//! when a full renderer cannot be initialized. In a full GUI environment
//! this is the place to add the code that initializes a Window + RenderState
//! and produces accurate screenshots.

use anyhow::Context;
use clap::Parser;
use std::path::PathBuf;

/// Generate baseline PNGs using the wezterm renderer.
#[derive(Parser, Debug)]
pub struct Args {
    /// Output directory for baselines
    #[arg(long, default_value = "tests/baselines")]
    out_dir: PathBuf,

    /// When set, attempt to initialize the real renderer. If not available,
    /// fall back to placeholders.
    #[arg(long)]
    force_renderer: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    eprintln!("Baseline generator (wezterm-gui).\nOutputs: {}", args.out_dir.display());

    std::fs::create_dir_all(&args.out_dir).context("create out_dir")?;

    // In the future: attempt to initialize Window + RenderState here and render
    // the samples using the real renderer. That requires a machine with GPU/
    // display access and may need to run inside CI with an appropriate runner.
    // For now we write placeholder PNGs so the harness can be exercised.

    if args.force_renderer {
        eprintln!("Attempting to initialize renderer... (not implemented in this helper yet)");
        eprintln!("Falling back to placeholders.");
    }

    write_placeholders(&args.out_dir)?;

    eprintln!("Wrote placeholder baselines to {}", args.out_dir.display());
    eprintln!("To produce accurate baselines, run this binary on a machine with a display and implement the renderer init in this helper.");

    Ok(())
}

fn write_placeholders(out_dir: &std::path::Path) -> anyhow::Result<()> {
    use image::{ImageBuffer, Rgba};

    let samples = [
        ("rtl_arabic.png", (0xff, 0xcc, 0xcc)),
        ("mixed_ltr_rtl.png", (0xcc, 0xff, 0xcc)),
        ("rtl_ligatures.png", (0xcc, 0xcc, 0xff)),
        ("rtl_double_width.png", (0xff, 0xff, 0xcc)),
        ("rtl_parentheses.png", (0xcc, 0xff, 0xff)),
    ];

    for (name, (r, g, b)) in &samples {
        let mut im: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_pixel(800, 64, Rgba([0, 0, 0, 0]));

        for x in 0..800 {
            for y in 16..48 {
                let p = im.get_pixel_mut(x, y);
                *p = Rgba([*r, *g, *b, 0xff]);
            }
        }

        let out = out_dir.join(name);
        im.save(&out).context("save placeholder png")?;
    }

    // Create a README marker describing how to run the generator in a full GUI.
    let marker = out_dir.join("README.txt");
    std::fs::write(
        &marker,
        "This directory contains placeholder baselines.\nRun this binary in a full GUI environment and implement the renderer init to generate accurate baselines.",
    )?;

    Ok(())
}
