use std::path::Path;

use clap::Parser;
use image::{imageops::crop, ImageError, ImageReader};

#[derive(Parser, Debug)]
struct Args {
    /// Sets crop point's X value
    #[arg(short, long, default_value_t = 0)]
    x: u32,

    /// Sets crop point's Y value
    #[arg(short, long, default_value_t = 0)]
    y: u32,

    /// Crop width, from the crop point
    #[arg(long)]
    width: Option<u32>,

    /// Crop height, from the crop point
    #[arg(long)]
    height: Option<u32>,

    /// Source file to read
    #[arg(short, long)]
    src: String,

    /// File to write new image to
    #[arg(short, long)]
    dst: String,

    /// If dst is RO, attempt to make it W
    #[arg(short, long)]
    force: bool
}

fn main() -> Result<(),ImageError>{
    let args = Args::parse();

    let target_path = Path::new(&args.dst);
    if let Ok(true) = std::fs::exists(&args.dst) {
        let mut perms = std::fs::metadata(&args.dst)?.permissions();
        if perms.readonly() && args.force {
            perms.set_readonly(false);
            std::fs::set_permissions(&args.dst, perms)?;
        }
    } else if let Some(parent) = target_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut img = ImageReader::open(args.src)?.decode()?;
    let w= args.width.unwrap_or(img.width());
    let h = args.height.unwrap_or(img.height());
    let subimg = crop(&mut img, args.x, args.y, w, h);
    subimg.to_image().save(args.dst)?;
    Ok(())
}
