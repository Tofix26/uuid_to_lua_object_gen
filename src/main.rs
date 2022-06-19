#![feature(path_try_exists)]
use json_comments::StripComments;
use serde_json::{from_reader, Value};
use std::fs::{create_dir_all, read_dir, read_to_string, try_exists, DirEntry, File};
use std::io::Write;
use std::vec;

struct Folder {
    path: String,
    file: String,
    entries: Vec<String>,
    set_list: String,
    set_entry: Option<String>,
}

impl Folder {
    fn new(
        path: &str,
        file: &str,
        entries: Vec<String>,
        set_list: &str,
        set_entry: Option<&str>,
    ) -> Folder {
        let set_entry = if set_entry.is_some() {
            Some(set_entry.unwrap().to_string())
        } else {
            None
        };
        Folder {
            path: path.to_string(),
            file: file.to_string(),
            entries,
            set_list: set_list.to_string(),
            set_entry: set_entry,
        }
    }
}
fn main() {
    create_dir_all("./Scripts/util").unwrap();
    let mut lua_file = File::create("./Scripts/util/uuids.lua").unwrap();
    let mut file_content =
        String::from("---@diagnostic disable: lowercase-global\n-- this file is generated\n");
    let folders = [
        Folder::new(
            "./Objects/Database",
            "shapesets.shapedb",
            vec!["partList".to_string(), "blockList".to_string()],
            "shapeSetList",
            None,
        ),
        Folder::new(
            "./ScriptableObjects",
            "scriptableObjectSets.sobdb",
            vec!["scriptableObjectList".to_string()],
            "scriptableObjectSetList",
            Some("scriptableObjectSet"),
        ),
        Folder::new(
            "./Harvestables/Database",
            "harvestablesets.harvestabledb",
            vec!["harvestableList".to_string()],
            "harvestableSetList",
            Some("name"),
        ),
        Folder::new(
            "./Tools/Database",
            "toolsets.tooldb",
            vec!["toolList".to_string()],
            "toolSetList",
            None,
        ),
        Folder::new(
            "./Characters/Database",
            "charactersets.characterdb",
            vec!["characters".to_string()],
            "characterSetList",
            None,
        ),
    ];
    for folder in folders.iter() {
        if let Err(..) = try_exists(&folder.path) {
            println!("Folder {} not found skipping...", folder.path);
            continue;
        }
        let files: Vec<DirEntry> = read_dir(&folder.path)
            .unwrap()
            .into_iter()
            .filter_map(|v| {
                let v = v.unwrap();
                if v.path().is_file() && v.file_name().to_str().unwrap().to_string() == folder.file
                {
                    Some(v)
                } else {
                    None
                }
            })
            .collect();
        if files.len() < 1 {
            println!("{} file not found skipping {}", folder.file, folder.path);
            continue;
        }
        file_content += &format!("\n----------------------------------------\n-- {}\n----------------------------------------\n", folder.file);
        let file = &read_to_string(files.iter().last().unwrap().path()).unwrap();
        let stripped = StripComments::new(file.as_bytes());
        let file: Value = from_reader(stripped).unwrap();
        for mut entry in file[&folder.set_list].as_array().unwrap().iter() {
            let set_entry = folder.set_entry.as_ref();
            if set_entry.is_some() {
                entry = &entry[set_entry.unwrap()];
            }
            let path = ".".to_string() + &entry.to_string().split_at(14).1.replace("\"", "");
            let name = &entry.as_str().unwrap().split("/").last().unwrap();
            file_content += &gen_set(name, path, folder);
        }
    }
    let name = "projectiles.projectileset";
    let path = "./Projectiles/projectiles.projectileset";
    let folder = Folder::new(
        "doesn't matter",
        "doesn't matter",
        vec!["projectiles".to_string()],
        "doesn't matter",
        None,
    );
    file_content += &gen_set(name, path.to_string(), &folder);
    lua_file.write(file_content.as_bytes()).unwrap();
}

fn gen_set(name: &str, path: String, folder: &Folder) -> String {
    let mut file_content = String::from(format!("\n----------------------------------------\n-- {}\n----------------------------------------\n", name));
    let file = &read_to_string(path).unwrap();
    let stripped = StripComments::new(file.as_bytes());
    let file: Value = from_reader(stripped).unwrap();
    for entry in folder.entries.iter() {
        if file[entry].is_null() {
            continue;
        }
        file_content += &format!("\n-- {}\n\n", entry);
        for object in file[entry].as_array().unwrap().iter() {
            if object["uuid"].is_null() {
                println!("There is an object without uuid in file {}", name);
                continue;
            }
            if object["name"].is_null() || object["name"].as_str().unwrap().is_empty() {
                println!(
                    "Object {} in file {} doesn't have a name",
                    object["uuid"], name
                );
                continue;
            }
            file_content += &format!(
                "{} = sm.uuid.new(\"{}\")\n",
                object["name"].as_str().unwrap(),
                object["uuid"].as_str().unwrap()
            );
        }
    }
    file_content
}
