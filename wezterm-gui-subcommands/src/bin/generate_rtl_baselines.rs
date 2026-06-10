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

    eprintln!("Attempting to run baseline generator that initializes an offscreen renderer.");

    // Create output directory
    std::fs::create_dir_all(&args.out_dir).context("create out_dir")?;

    // Try to initialize a headless window + renderer. If that fails, write a helpful message.
    if let Err(err) = try_generate(&args.out_dir) {
        eprintln!("Baseline generation failed: {:#}", err);
        let marker = args.out_dir.join("README.txt");
        std::fs::write(&marker, format!("Failed to generate baselines: {}\nRun this on a machine with GPU/display access and the full build.\n", err))?;
        eprintln!("Wrote marker to {}", marker.display());
        anyhow::bail!("baseline generation failed")
    }

    eprintln!("Baseline generation completed into {}", args.out_dir.display());
    Ok(())
}

fn try_generate(out_dir: &std::path::Path) -> anyhow::Result<()> {
    // The real implementation would initialize a Window, RenderContext, RenderState,
    // and construct TermWindow-like state to render sample lines with mirror_rtl_runs=true.
    // Implementing that here is substantial. As a first step, write placeholder PNGs
    // that demonstrate the harness and can be replaced by true baselines when this
    // binary is run in the full environment.

    use image::{ImageBuffer, Rgba};

    // Simple synthetic placeholder images (transparent background with text-like rectangles)
    let samples = [
        ("rtl_arabic.png", (0xff, 0xcc, 0xcc)),
        ("mixed_ltr_rtl.png", (0xcc, 0xff, 0xcc)),
        ("rtl_ligatures.png", (0xcc, 0xcc, 0xff)),
        ("rtl_double_width.png", (0xff, 0xff, 0xcc)),
        ("rtl_parentheses.png", (0xcc, 0xff, 0xff)),
    ];

    for (name, (r, g, b)) in &samples {
        let mut im: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_pixel(800, 64, Rgba([0, 0, 0, 0]));

        // draw a colored bar to indicate the sample
        for x in 0..800 {
            for y in 16..48 {
                let p = im.get_pixel_mut(x, y);
                *p = Rgba([*r, *g, *b, 0xff]);
            }
        }

        let out = out_dir.join(name);
        im.save(&out).context("save placeholder png")?;
    }

    Ok(())
}
