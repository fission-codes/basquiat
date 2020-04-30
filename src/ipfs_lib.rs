use std::process::{Command, Stdio};
use crate::rendered::{RenderedImage, MultiImage};
use std::io::{Write, Read};

impl RenderedImage {

    pub fn add(&mut self) -> (){
        self.cid = Some(add_buffer(self.buffer.as_slice()));
    }
}

impl MultiImage {
    pub fn append(&mut self, child: &mut RenderedImage) -> () {
        match child.cid {
            Some(_) => (),
            None => child.add()
        }
        // println!("Current cid: {}", &self.cid);
        let child_cid = child.cid.as_ref().unwrap();
        let filenames = child.filenames();
        for filename in filenames.iter() {
            self.add_link(filename, child_cid);
        }
    }

    fn add_link(&mut self, filename: &str, child_cid: &str){
        let update_arguments = ["object", "patch", "add-link"];
        let new_cid = Command::new("ipfs").args(&update_arguments)
            .arg(&self.cid)
            .arg("--")
            .arg(filename)
            .arg(child_cid)
            .output()
            .expect(&format!("Failed to add link to {}", &filename));
        // println!("{}", String::from_utf8(new_cid.stderr).unwrap());
        self.cid = String::from(String::from_utf8(new_cid.stdout).expect("Failed to parse ipfs command output").trim());
        self.links.push(String::from(filename))
    }

    pub fn generate_html(&mut self){
        let mut output_text = String::new();
        for link in self.links.iter(){
            output_text = format!("{}\n<br><a href=\"{}\">{}</a>", &output_text, link, link)
        }
        let cid = add_buffer(output_text.as_bytes());
        self.add_link("thumbnails.html", &cid);
    }
}

fn add_buffer(buffer: &[u8]) -> String {
    let add_arguments = ["add", "-Q"];
    let process = Command::new("ipfs").args(&add_arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect(&format!("Failed adding to ipfs"));
    match process.stdin.unwrap().write_all(buffer) {
        Err(why) => panic!("couldn't write to stdin : {:?}", why),
        Ok(_) => ()
    }
    let mut cid = String::new();
    match process.stdout.unwrap().read_to_string(&mut cid){
        Err(why) => panic!("couldn't read stdout : {:?}", why),
        Ok(_) => ()
    };
    String::from(cid.trim())
}
