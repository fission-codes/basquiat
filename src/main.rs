use std::{path::Path};
mod ipfs_lib;
mod img_lib;
mod rendered;
use crate::img_lib::Resizable;
use crate::rendered::MultiImage;
use clap::Clap;


#[derive(Clap)]
#[clap(version="0.1")]
struct Opts {
    /// path to image file
    file_path: String
}

fn main() {
    let opts : Opts = Opts::parse();

    let img_path = Path::new(&opts.file_path);
    let img_path_str = img_path.to_str().unwrap();

    let target_heights = [50, 100, 150];

    let _app = img_lib::init();

    let image = img_lib::ImageLibVips::new(img_path_str);
    let output = batch_resize_buffer(&image, &target_heights);
    println!("{}", &output.cid);


}

fn _batch_resize<T : Resizable>(image : &T, directory : &str, target_scales : &[f64]){
    // image.write(&format!("{}/original.jpg", directory));

    for target in target_scales.iter() {
        image.resize(*target, directory);
    }
}


fn batch_resize_buffer<T : Resizable>(image : &T, target_heights : &[i32]) -> MultiImage {
    let mut original = image.render_original();
    original.add();
    let mut root = MultiImage { cid: original.cid.unwrap()};

    for target in target_heights.iter() {
        let mut rendered = image.render_height(*target);
        root.append(&mut rendered);
    }
    root
}