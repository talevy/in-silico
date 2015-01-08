#![feature(plugin)]

#[plugin] extern crate docopt_macros;
extern crate "rustc-serialize" as rustc_serialize;
extern crate serialize;
extern crate docopt;
extern crate image;
extern crate img_hash;
extern crate glob;

use std::sync::Future;
use img_hash::ImageHash;
use glob::glob;
use multipaths::MultiPaths;

mod multipaths;

fn diff(img_path1: &Path, img_path2: &Path) -> f32 {
    let image1 = image::open(img_path1).unwrap();
    let image2 = image::open(img_path2).unwrap();

    let hash1 = ImageHash::hash(&image1, 8, false);
    let hash2 = ImageHash::hash(&image2, 8, false);
    hash1.dist_ratio(&hash2)
}

fn differ(image_paths: &Vec<Path>) {
    for (i, p1) in image_paths.init().iter().enumerate() {
        for p2 in image_paths.slice_from(i + 1).iter() {
            println!("{}", diff(p1, p2));
        }
    }
}

fn differ_par(image_paths: &Vec<Path>) {
    let mut futures: Vec<Vec<Future<f32>>> = image_paths.init().iter().enumerate().map(|(i, p1)| {
        image_paths.slice_from(i + 1).iter().map(|p2| {
            let (a, b) = (p1.clone(), p2.clone());
            Future::spawn(move|| { diff(&a, &b) })
        }).collect()
    }).collect();

    for fts in futures.iter_mut()  {
        for ft in fts.iter_mut() {
            println!("{}", ft.get());
        }
    }
}

docopt!(Args derive Show, "
Usage: pd [options] <img>...
       pd --help

Options:
 -h, --help       Show this message.
 -p, --parallel   execute diffs in parallel
");
fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let paths: MultiPaths = MultiPaths::from(args.arg_img.iter().map(|p| glob(p.as_slice())).collect());

    let image_paths: Vec<Path> = paths.collect();

    if args.flag_parallel {
        differ_par(&image_paths);
    } else {
        differ(&image_paths);
    }
}

