pub struct RenderedImage {
    pub buffer : Vec<u8>,
    pub width : i32,
    pub height : i32,
    pub extension : String,
    pub cid : Option<String>
}

pub struct MultiImage {
    pub cid : String
}

impl RenderedImage {
    pub fn filenames(&self) -> [String; 3]{
        [format!("{}x{}.{}", self.width, self.height, &self.extension),
        format!("{}x_.{}", self.width, &self.extension),
        format!("_x{}.{}", self.height, &self.extension)]
    }
    pub fn _filename(&self) -> String{
        format!("{}x{}.{}", self.width, self.height, &self.extension)
    }
}