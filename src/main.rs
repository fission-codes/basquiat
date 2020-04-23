use std::{path::Path,fs, io::Read};
mod ipfs_lib;
mod img_lib;
mod rendered;
use crate::img_lib::Resizable;
use crate::rendered::MultiImage;
use clap::Clap;


#[derive(Clap)]
#[clap(version="0.1")]
struct Opts {
    /// Outputs all versions to a thumbnails directory in the current directory
    #[clap(short = "f", long = "use-filesystem")]
    fs: bool,
    /// path to image file
    file_path: String
}

fn main() {
    let opts : Opts = Opts::parse();

    // let args: Vec<String> = env::args().collect();
    // if args.len() != 2 {
    //     println!("Invalid arguments");
    //     print_help();
    //     return;
    // }

    let img_path = Path::new(&opts.file_path);
    let img_ext = img_path.extension().expect("Invalid path entered").to_str().unwrap();
    let img_path_str = img_path.to_str().unwrap();

    let target_scales = [0.5, 0.25, 0.3];

    let _app = img_lib::init();

    let image = img_lib::ImageLibVips::new(img_path_str);
    if opts.fs {
        let output_dir = "thumbnails";
        let img_name = img_path.file_name().unwrap().to_str().unwrap();
        let current_dir = format!("{}/{}", output_dir, img_name);

        fs::create_dir_all(&current_dir).expect("Failed to create output directory");


        batch_resize(&image, &current_dir, &target_scales, &img_ext);
        let versions = Path::new(&current_dir)
            .read_dir().expect("Failed to open temp output directory");
        let cid = ipfs_lib::store(img_path_str, versions);
        println!("{}", &cid);
    } else {
        let output = batch_resize_buffer(&image, &target_scales, img_path_str);
        println!("{}", &output.cid);
    }


}

fn batch_resize<T : Resizable>(image : &T, directory : &str, target_scales : &[f64], img_ext : &str){
    // image.write(&format!("{}/original.jpg", directory));

    for target in target_scales.iter() {
        image.resize(*target, directory, img_ext);
    }
}


fn batch_resize_buffer<T : Resizable>(image : &T, target_scales : &[f64], original_path : &str) -> MultiImage {
    let mut original_file = fs::File::open(original_path).unwrap();
    let mut original_buffer = Vec::new();
    original_file.read_to_end(&mut original_buffer).unwrap();

    let mut original = image.render_original();
    original.add();
    let mut root = MultiImage { cid: original.cid.unwrap()};

    for target in target_scales.iter() {
        let mut rendered = image.render_size(*target);
        root.append(&mut rendered);
    }
    root
}