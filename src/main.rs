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
    // let cid = Command::new("ipfs")
    //     .args(&["add", "-r", "-Q"])
    //     .arg(current_dir)
    //     .output()
    //     .expect("Failed to add to ipfs");
    // let cid = String::from_utf8(cid.stdout).expect("Failed to parse stdout from ipfs add command");
    let versions = Path::new(&current_dir)
        .read_dir().expect("Failed to open temp output directory");
    let cid = ipfs_lib::store(img_path_str, versions);
    println!("Content at : {}", &cid);
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


mod ipfs_lib {
    use std::{process::Command, path::Path, fs::ReadDir};

    pub fn store(original_filepath : &str, version_paths: ReadDir) -> String{
        let add_arguments = ["add", "-r", "-Q"];
        let update_arguments = ["object", "patch", "add-link"];
        let cid_original = Command::new("ipfs").args(&add_arguments).arg(original_filepath)
            .output()
            .expect(&format!("Failed adding {} version to ipfs", original_filepath));
        let mut cid_current = String::from(String::from_utf8(cid_original.stdout).expect("Failed to parse ifps command output").trim());
        for entry in version_paths{
            let path_obj = entry.unwrap().path();
            let path = path_obj.to_str().unwrap();
            let filename = Path::new(path).file_name().unwrap().to_str().unwrap();

            let cid_version = Command::new("ipfs").args(&add_arguments).arg(path)
                .output()
                .expect(&format!("Failed adding {} version to ipfs", path));


            let cid_version = String::from(String::from_utf8(cid_version.stdout).expect("Failed to parse ipfs command output").trim());
            let cid_current_output = Command::new("ipfs").args(&update_arguments)
                .arg(&cid_current)
                .arg(&filename)
                .arg(&cid_version)
                .output()
                .expect(&format!("Failed to add link to {}", &filename));
            cid_current = String::from(String::from_utf8(cid_current_output.stdout).expect("Failed to parse ipfs command output").trim());
        }
        cid_current
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