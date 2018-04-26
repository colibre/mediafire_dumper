#![feature(custom_attribute)]
#![feature(nll)]
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate reqwest;
extern crate url;

#[macro_use]
mod api;

use serde_json::Value;
use std::io::Write;
use url::{ParseError, Url};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct Folder {
    folderkey: String,
    name: String,
}
impl Folder {
    fn new<T: Into<String>>(name: T, folderkey: T) -> Folder {
        Folder {
            name: name.into(),
            folderkey: folderkey.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct File {
    #[serde(rename = "filename")]
    name: String,
    quickkey: String,
}

fn main() {
    let argument = std::env::args().skip(1).next().expect("No argument given");
    let client = reqwest::Client::new();

    match Url::parse(&argument) {
        // TODO: Properly return errors and nodes to the result
        Ok(url) => {
            if let Some(folder_key) = url.fragment() {
                let params = [("response_format", "json"), ("folder_key", &folder_key)];
                let v = api::folder::get_info(Some(&client), &params).unwrap();

                let name = v["response"]["folder_info"]["name"].as_str().unwrap();
                let folder_tree = Node::new(Folder::new(name, folder_key));
                folder_tree.print(0);
            } else {
                //it's a link without the fragment, call the API to get the folder_key then continue if possible
            }
        }
        Err(err) => {
            let argument = argument.trim_left_matches('#');
            let folder_key = if argument.is_alphanumeric() {
                Some(argument)
            } else {
                None
            };
            match folder_key {
                Some(folder_key) => {
                    let params = [("response_format", "json"), ("folder_key", &folder_key)];
                    let v = api::folder::get_info(Some(&client), &params).unwrap();

                    if v["response"]["result"].as_str() == Some("Success") {
                        let name = v["response"]["folder_info"]["name"].as_str().unwrap();
                        let folder_tree = Node::new(Folder::new(name, folder_key));
                        folder_tree.print(0);
                    } else {
                        eprintln!("Incorrect folder key");
                    }
                }
                None => {}
            }
        }
    }
    //let folder_tree = Node::new(Folder::new("Denpa", "xczuuk44mz3hv"));
    //let folder_tree = Node::new(Folder::new("Denpa", "2kww1wa95c61d")); //temporarily hardcoded ecks dee
    //folder_tree.print(0).expect("IO Error occured");
}

struct Node {
    folder: Folder,
    nodes: Vec<Node>,
    files: Vec<File>,
}

impl Node {
    fn new(folder: Folder) -> Node {
        let http_client = reqwest::Client::new();
        let mut nodes = Vec::new();
        let folders = get_folders(&folder, &http_client);
        if !folders.is_empty() {
            for folder in folders {
                let node = Node::new(folder);
                nodes.push(node);
            }
        }

        Node {
            folder: folder.clone(),
            nodes,
            files: get_files(&folder, &http_client),
        }
    }

    fn print(self, pad: u16) -> Result<(), std::io::Error> {
        let mut stdout = std::io::BufWriter::new(std::io::stdout());
        for _n in 0..pad {
            stdout.write("== ".as_bytes())?;
        }
        stdout.write(format!("{}:\t {}", self.folder.name, self.folder.folderkey).as_bytes())?;
        stdout.write(&[b'\n'])?;
        if !self.nodes.is_empty() {
            for node in self.nodes {
                node.print(pad + 1)?;
            }
        }
        for file in self.files {
            for _n in 0..pad {
                stdout.write("== ".as_bytes())?;
            }
            stdout.write("=> ".as_bytes())?;
            stdout.write(file.name.as_bytes())?;
            stdout.write(&[b'\n'])?;
        }
        Ok(())
    }
    #[allow(dead_code)]
    fn add_node(mut self, node: Node) {
        self.nodes.push(node);
    }
    #[allow(dead_code)]
    fn add_file(mut self, file: File) {
        self.files.push(file);
    }
}

fn get_files(folder: &Folder, client: &reqwest::Client) -> Vec<File> {
    let mut files: Vec<File> = vec![];
    for n in 1.. {
        let params = [
            ("content_type", "files"),
            ("response_format", "json"),
            ("folder_key", &folder.folderkey),
            ("chunk", &n.to_string()),
        ];
        let v = api::folder::get_content(Some(client), &params).unwrap();

        let temp: Vec<File> = serde_json::from_value(
            v["response"]["folder_content"]["files"].clone(),
        ).expect("Failed parsing");
        if temp.is_empty() {
            break;
        }
        files = files.into_iter().chain(temp.into_iter()).collect();
    }
    files
}

fn get_folders(folder: &Folder, client: &reqwest::Client) -> Vec<Folder> {
    let mut folders: Vec<Folder> = vec![];
    for n in 1.. {
        let params = [
            ("content_type", "folders"),
            ("response_format", "json"),
            ("folder_key", &folder.folderkey),
            ("chunk", &n.to_string()),
        ];
        let v = api::folder::get_content(Some(client), &params).unwrap();

        let temp: Vec<Folder> = serde_json::from_value(
            v["response"]["folder_content"]["folders"].clone(),
        ).unwrap_or_default();
        if temp.is_empty() {
            break;
        }
        folders = folders.into_iter().chain(temp.into_iter()).collect();
    }
    folders
}
