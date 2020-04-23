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
    pub fn filename(&self) -> String{
        format!("{}_{}.{}", self.width, self.height, &self.extension)
    }
}