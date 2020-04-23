use std::{process::{Command, Stdio}, path::Path, fs::ReadDir};
use crate::rendered::{RenderedImage, MultiImage};
use std::io::{Write, Read};

pub fn store(original_filepath : &str, version_paths: ReadDir) -> String{
    let add_arguments = ["add", "-Q"];
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

impl RenderedImage {

    pub fn add(&mut self) -> (){
        let add_arguments = ["add", "-Q"];
        let process = Command::new("ipfs").args(&add_arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect(&format!("Failed adding to ipfs"));
        match process.stdin.unwrap().write_all(self.buffer.as_slice()) {
            Err(why) => panic!("couldn't write to stdin : {:?}", why),
            Ok(_) => ()
        }
        let mut cid = String::new();
        match process.stdout.unwrap().read_to_string(&mut cid){
            Err(why) => panic!("couldn't read stdout : {:?}", why),
            Ok(_) => ()
        };
        self.cid = Some(String::from(cid.trim()));
    }
}

impl MultiImage {
    pub fn append(&mut self, child: &mut RenderedImage) -> (){
        match child.cid {
            Some(_) => (),
            None => child.add()
        }
        // println!("Current cid: {}", &self.cid);
        let update_arguments = ["object", "patch", "add-link"];
        let filename = child.filename();
        let child_cid = child.cid.as_ref().unwrap();
        let new_cid = Command::new("ipfs").args(&update_arguments)
            .arg(&self.cid)
            .arg(&filename)
            .arg(child_cid)
            .output()
            .expect(&format!("Failed to add link to {}", &filename));
        // println!("Added {}", &filename);
        self.cid = String::from(String::from_utf8(new_cid.stdout).expect("Failed to parse ipfs command output").trim());
    }

    pub fn _append_original(&mut self, child: &mut RenderedImage) -> (){
        match child.cid {
            Some(_) => (),
            None => child.add()
        }
        // println!("Current cid: {}", &self.cid);
        let update_arguments = ["object", "patch", "add-link"];
        let filename = String::from("");
        let child_cid = child.cid.as_ref().unwrap();
        let new_cid = Command::new("ipfs").args(&update_arguments)
            .arg(&self.cid)
            .arg(&filename)
            .arg(child_cid)
            .output()
            .expect(&format!("Failed to add link to {}", &filename));
        // println!("Added {}", &filename);
        self.cid = String::from(String::from_utf8(new_cid.stdout).expect("Failed to parse ipfs command output").trim());
    }
}



