// use libvips::{ops, VipsImage, VipsApp};
use std::{env, fs, process::Command, path::Path};
// use std::thread::current;
// use libvips::error::Error::ResizeError;
use crate::img_lib::Resizable;


fn main() {
    let output_dir = "thumbnails";

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid arguments");
        print_help();
        return;
    }

    let img_path = Path::new(&args[1]);
    let img_ext = img_path.extension().expect("Invalid path entered").to_str().unwrap();
    let img_name = img_path.file_name().unwrap().to_str().unwrap();
    let img_path_str = img_path.to_str().unwrap();

    let target_scales = [0.5, 0.25, 0.3];

    let _app = img_lib::init();

    let current_dir = format!("{}/{}", output_dir, img_name);

    fs::create_dir_all(&current_dir).expect("Failed to create output directory");

    let image = img_lib::ImageLibVips::new(img_path_str);

    batch_resize(&image, &current_dir, &target_scales, &img_ext);

    // image.image_write_to_file(&format!("{}/original.jpg", &current_dir)).expect("Failed to write original");
    //
    // for target in target_scales.iter() {
    //     let resized = ops::resize(&image, *target).expect("Resize failed");
    //     resized.image_write_to_file(&format!("{}/{}.{}", &current_dir, target, img_ext)).expect("Failed to write output");
    // }
    Command::new("ipfs")
        .arg("add")
        .arg("-r")
        .arg(current_dir)
        .status()
        .expect("Failed to add to ipfs");
}

fn print_help(){
    println!("usage : iimir <path_to_file>")
}

fn batch_resize<T : Resizable>(image : &T, directory : &str, target_scales : &[f64], img_ext : &str){
    image.write(&format!("{}/original.jpg", directory));

    for target in target_scales.iter() {
        image.resize(*target, &format!("{}/{}.{}", directory, target, img_ext))
    }
}





mod img_lib {
    use libvips::{ops, VipsImage, VipsApp};

    pub trait Resizable {

        fn new(path_to_image : &str) -> Self;
        fn resize(&self, target_scale : f64, target_path : &str);
        fn write(&self, target_path : &str);
    }

    pub struct ImageLibVips {
        data : VipsImage
    }

    pub fn init() -> VipsApp{
        let app=VipsApp::new("mainVips", false).expect("Failed to initialize libvips!");
        app.concurrency_set(2);
        return app;
    }

    impl Resizable for ImageLibVips {
        fn new(path_to_image : &str) -> ImageLibVips {
             ImageLibVips{
                data : VipsImage::new_from_file(path_to_image).expect("Couldn't open file")
            }
        }

        fn resize(&self, target_scale : f64, target_path : &str) {
            let resized = ops::resize(&self.data, target_scale).expect("Resize failed");
            resized.image_write_to_file(target_path).expect("Failed to write output");
        }

        fn write(&self, target_path : &str){
            self.data.image_write_to_file(target_path).expect("Failed to write original");
        }
    }

}