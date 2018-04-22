#![feature(custom_attribute)]
#![feature(nll)]
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate reqwest;

use std::collections::BTreeMap;
use serde_json::Value;
use std::string::ToString;
use std::io::Write;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct Folder {
    folderkey: String,
    name: String,
}
impl Folder {
    fn new<T: AsRef<str> + ToString>(name: T, folderkey: T) -> Folder {
        Folder {
            name: name.to_string(),
            folderkey: folderkey.to_string(),
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
//    let mut request = reqwest::get("http://www.mediafire.com/api/1.1/folder/get_content.php?folder_key=xczuuk44mz3hv&response_format=json").unwrap();
//    let v: Value = serde_json::from_str(&request.text().unwrap()).unwrap();
    let folder_tree = Node::new(Folder::new("Denpa", "xczuuk44mz3hv"));
    folder_tree.print(0).expect("IO Error occured");
    //let folders: Vec<Folder> = serde_json::from_value(v["response"]["folder_content"]["folders"].clone()).unwrap();
    //let serialized = serde_json::to_string_pretty(&folders).unwrap();
    //println!("{}", serialized);
    //println!("Hello, world!");
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
        for _n in 0..pad { stdout.write("== ".as_bytes())?; }
        stdout.write(format!("{}:\t {}", self.folder.folderkey, self.folder.name).as_bytes())?;
        stdout.write(&[b'\n'])?;
        if !self.nodes.is_empty() {
            for node in self.nodes {
                node.print(pad+1)?;
            }
        }
        for file in self.files {
            for _n in 0..pad { stdout.write("== ".as_bytes())?; }
            stdout.write("=> ".as_bytes())?;
            stdout.write(file.name.as_bytes())?;
            stdout.write(&[b'\n'])?;
        }
        Ok(())
    }
    fn add_node(mut self, node: Node) {
        self.nodes.push(node);
    }
    fn add_file(mut self, file: File) {
        self.files.push(file);
    }
}

fn get_files(folder: &Folder, client: &reqwest::Client) -> Vec<File> {
    let mut files: Vec<File> = vec![];
    for n in 1.. {
        let mut request = client.get(&format!("http://www.mediafire.com/api/1.1/folder/get_content.php?folder_key={}&content_type=files&response_format=json&chunk={}",
                                               folder.folderkey,
                                               n)).send().expect("Failed requesting files");
        
        let v: Value = serde_json::from_str(&request.text().unwrap()).unwrap_or_default();

        let temp: Vec<File> = serde_json::from_value(v["response"]["folder_content"]["files"].clone()).unwrap();
        if temp.is_empty() { break; }
        files = files.into_iter().chain(temp.into_iter()).collect();
    }
    files
}

fn get_folders(folder: &Folder, client: &reqwest::Client) -> Vec<Folder> {
    let mut folders: Vec<Folder> = vec![];
    for n in 1.. { // requests in chunks, see link or API documentation
        let mut request = client.get(&format!("http://www.mediafire.com/api/1.1/folder/get_content.php?folder_key={}&content_type=folders&response_format=json&chunk={}",
                                               folder.folderkey,
                                               n)).send().expect("Failed requesting folders");
        let v: Value = serde_json::from_str(&request.text().unwrap()).unwrap_or_default();

        let temp: Vec<Folder> = serde_json::from_value(v["response"]["folder_content"]["folders"].clone()).unwrap_or_default();
        if temp.is_empty() { break; }
        folders = folders.into_iter().chain(temp.into_iter()).collect();
    }
    folders
}

// Folder
// |     \
// FolderFolder
// etc.