use libvips::{ops, VipsImage, VipsApp};
use crate::rendered::RenderedImage;
use std::path::Path;
use std::fs;
use std::io::Read;

pub trait Resizable {

    fn new(path_to_image : &str) -> Self;
    fn resize(&self, target_scale : f64, directory : &str, img_ext : &str);
    fn write(&self, target_path : &str);
    fn render(&self) -> RenderedImage;
    fn render_original(&self) -> RenderedImage;
    fn render_size(&self, target_scale : f64) -> RenderedImage;
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
}

pub struct ImageLibVips {
    data : VipsImage,
    original_path : String,
    extension : String
}

pub fn init() -> VipsApp{
    let app=VipsApp::new("mainVips", false).expect("Failed to initialize libvips!");
    app.concurrency_set(2);
    return app;
}

impl Resizable for ImageLibVips {
    fn new(path_to_image : &str) -> ImageLibVips {
        let img_path = Path::new(path_to_image);
        ImageLibVips{
            data : VipsImage::new_from_file(path_to_image).expect("Couldn't open file"),
            original_path : String::from(path_to_image),
            extension : String::from(img_path.extension().unwrap().to_str().unwrap())
        }
    }

    fn resize(&self, target_scale : f64, directory : &str, img_ext : &str) {
        let resized = ops::resize(&self.data, target_scale).expect("Resize failed");
        resized.image_write_to_file(&format!("{}/{}_{}.{}", directory, resized.get_width(), resized.get_height(), img_ext)).expect("Failed to write output");
    }

    fn write(&self, target_path : &str){
        self.data.image_write_to_file(target_path).expect("Failed to write original");
    }

    fn render(&self) -> RenderedImage{
        // let buffer = self.data.image_write_to_buffer(extension).expect("Failed to output original");
        let buffer = self.data.image_write_to_buffer(&".jpg").expect("Failed to render image");
        RenderedImage{
            buffer,
            width: self.data.get_width(),
            height: self.data.get_height(),
            extension: self.extension.clone(),
            cid: None,
        }
    }

    fn render_original(&self) -> RenderedImage {
        let mut original_file = fs::File::open(&self.original_path).unwrap();
        let mut original_buffer = Vec::new();
        original_file.read_to_end(&mut original_buffer).unwrap();
        RenderedImage{
            buffer: original_buffer,
            width: self.data.get_width(),
            height: self.data.get_height(),
            extension: self.extension.clone(),
            cid: None,

        }
    }

    fn render_size(&self, target_scale : f64) -> RenderedImage{
        let resized = ops::resize(&self.data, target_scale).expect("Resize failed");
        let resized = ImageLibVips{
            data: resized,
            original_path: self.original_path.clone(),
            extension: self.extension.clone()
        };
        resized.render()
    }

    fn get_width(&self) -> i32{
        self.data.get_width()
    }

    fn get_height(&self) -> i32{
        self.data.get_height()
    }
}