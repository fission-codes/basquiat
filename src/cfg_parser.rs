use std::{fs::File, path::Path, io::{BufReader, BufRead}};
use regex::Regex;

pub struct Config{
    pub dimensions: Resize,
    // pub operations: Option<Vec<Operation>>
}

pub struct Parser{
    re: Regex
}

pub enum Resize{
    Width(i32),
    Height(i32),
    Original
}

// pub struct Operation {
//     identifier: String,
//     parameters: Vec<Parameter>
// }
//
// pub enum Parameter {
//     Integer(i32),
//     Text(String)
// }

impl Parser {
    pub fn new() -> Parser{
        let re: Regex = Regex::new(r"(?P<width>\d+|_)x(?P<height>\d+|_)").unwrap();
        Parser{re}
    }

    pub fn parse_file(&self, filepath: &Path) -> Vec<Config>{
        let file = File::open(filepath).expect(&"cfg file not found");
        let lines = BufReader::new(file).lines();
        let mut configs : Vec<Config> = Vec::new();
        for line in lines {
            match self.parse_from_string(&line.unwrap()) {
                Some(conf) => configs.push(conf),
                None => ()
            }
        }
        configs
    }

    pub fn parse_from_string(&self, line: &str) -> Option<Config>{
        let first_char = line.chars().next();
        match first_char {
            Some('#') => return None,
            None => return None,
            Some(_) => ()
        }
        let matched= self.re.captures(line).expect("Syntax Error in cfg file");
        let width = matched.name("width").unwrap().as_str();
        let height = matched.name("height").unwrap().as_str();
        if width == "_" && height == "_"{
            return Some(Config{dimensions: Resize::Original})
        }
        if width == "_" {
            return Some(Config{dimensions: Resize::Height(height.parse::<i32>().unwrap())})
        }
        return Some(Config{dimensions: Resize::Width(width.parse::<i32>().unwrap())})
    }
}