use std::{path::Path};
mod ipfs_lib;
mod img_lib;
mod rendered;
mod cfg_parser;
use img_lib::Resizable;
use rendered::MultiImage;
use clap::Clap;
use cfg_parser::Config;


#[derive(Clap)]
#[clap(version="0.1")]
struct Opts {
    /// path to config file
    #[clap(short="c", long="config", default_value="basquiat.cfg")]
    cfg_path: String,
    /// Don't include thumbnails.html
    #[clap(short="s", long="skip-html")]
    skip_html: bool,
    /// Only output raw CID
    #[clap(short="q", long="quiet")]
    quiet: bool,
    /// path to image file
    file_path: String
}

fn main() {
    let opts : Opts = Opts::parse();

    let cfg_path = Path::new(&opts.cfg_path);
    let config_parser = cfg_parser::Parser::new();
    let configs = config_parser.parse_file(cfg_path);

    let img_path = Path::new(&opts.file_path);
    let img_path_str = img_path.to_str().unwrap();

    let _app = img_lib::init();

    let image = img_lib::ImageLibVips::new(img_path_str);
    let mut output = batch_resize_buffer(&image, &configs);

    if !&opts.skip_html {
        output.generate_html();
    }
    if opts.quiet {
        println!("{}", &output.cid);
    } else {
        println!("http://localhost:8080/ipfs/{}", &output.cid);
    }


}

fn _batch_resize<T : Resizable>(image : &T, directory : &str, target_scales : &[f64]){
    // image.write(&format!("{}/original.jpg", directory));

    for target in target_scales.iter() {
        image.resize(*target, directory);
    }
}


fn batch_resize_buffer<T : Resizable>(image : &T, configs : &Vec<Config>) -> MultiImage {
    let mut original = image.render_original();
    original.add();
    let mut root = MultiImage::new(original.cid.unwrap());

    for target in configs.iter() {
        let mut rendered = image.render_config(target);
        root.append(&mut rendered);
    }
    root
}