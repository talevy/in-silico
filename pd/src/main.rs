#![feature(plugin)]

#[plugin] extern crate docopt_macros;
extern crate "rustc-serialize" as rustc_serialize;
extern crate serialize;
extern crate docopt;
extern crate image;
extern crate img_hash;

use img_hash::ImageHash;

docopt!(Args derive Show, "
Usage: pd [options] <img>...
       pd --help

Options:
 -h, --help       Show this message.
");

fn diff(img_path1: &Path, img_path2: &Path) -> f32 {
    let image1 = image::open(img_path1).unwrap();
    let image2 = image::open(img_path2).unwrap();

    let hash1 = ImageHash::hash(&image1, 8, false);
    let hash2 = ImageHash::hash(&image2, 8, false);
    println!("Image1 hash: {}", hash1.to_base64());
    println!("Image2 hash: {}", hash2.to_base64());
    hash1.dist_ratio(&hash2)
}
fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let image_paths = args.arg_img;
    for (i, p1) in image_paths.init().iter().enumerate() {
        for p2 in image_paths.slice_from(i + 1).iter() {
            println!("({}, {}) - {}", p1, p2, diff(&Path::new(p1), &Path::new(p2)));
        }
    }
}

