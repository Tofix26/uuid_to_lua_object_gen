use std::{
    fs::{self, File},
    path::Path,
};

use colored::*;
use json_comments::StripComments;

mod structs;
use structs::{Databse, DbType, Entries, Folder, Set};

const UTIL_PATH: &str = "./Scripts/util";
const FOLDERS: [Folder; 5] = [
    Folder {
        path: "./Objects/Database/",
        file: "shapesets.shapedb",
        entries: ["partList", "blockList"],
        set_list: "shapeSetList",
        set_entry: None,
    },
    Folder {
        path: "./ScriptableObjects/",
        file: "scriptableObjectSets.sobdb",
        entries: ["scriptableObjectList", ""],
        set_list: "scriptableObjectSetList",
        set_entry: Some("scriptableObjectSet"),
    },
    Folder {
        path: "./Harvestables/Database/",
        file: "harvestablesets.harvestabledb",
        entries: ["harvestableList", ""],
        set_list: "harvestableSetList",
        set_entry: Some("name"),
    },
    Folder {
        path: "./Tools/Database/",
        file: "toolsets.tooldb",
        entries: ["toolList", ""],
        set_list: "toolSetList",
        set_entry: None,
    },
    Folder {
        path: "./Characters/Database/",
        file: "charactersets.characterdb",
        entries: ["characters", ""],
        set_list: "characterSetList",
        set_entry: None,
    },
];

fn main() {
    let path = Path::new(UTIL_PATH);

    if !path.exists() {
        fs::create_dir_all(path).unwrap();
    }

    File::create("./Scripts/util/uuids.lua").unwrap();
    let mut file_content =
        String::from("---@diagnostic disable: lowercase-global\n-- this file is generated\n");

    for folder in FOLDERS.iter() {
        let path = folder.path.to_string() + folder.file;
        let path = Path::new(&path);

        if !path.exists() {
            eprintln!(
                "{} | File {} not found skipping...",
                "INFO".yellow(),
                path.to_str().unwrap()
            );
            continue;
        };

        let db = fs::read_to_string(path).unwrap();
        let db = StripComments::new(db.as_bytes());
        let db: Databse = serde_json::from_reader(db).unwrap();

        let paths = db.index(&folder.set_list).as_ref().unwrap();
        let paths = match paths {
            DbType::Entry(entries) => Entries::parse_entries(entries, folder.set_entry.unwrap()),
            DbType::Data(data) => data.clone(),
        };

        for path in paths.iter() {
            let (_, path) = path.split_at(13);
            let path = ".".to_string() + path;
            let path = Path::new(&path);

            if !path.exists() {
                eprintln!("{} | Cannot find file {:?}", "Error".bright_red(), path);
                continue;
            }

            let json = fs::read_to_string(path).unwrap();
            let json = StripComments::new(json.as_bytes());
            let json: Set = serde_json::from_reader(json).unwrap();

            let file_name: Vec<&str> = path.to_str().unwrap().split("/").collect();
            let file_name = file_name[file_name.len() - 1];
            file_content += &format!("\n-- {file_name}\n\n");

            for entry in folder.entries.iter() {
                if entry.is_empty() {
                    continue;
                }
                let data = json.index(entry).as_ref();
                if data.is_none() {
                    continue;
                }
                let data = data.unwrap();

                for data in data.iter() {
                    if data.name.is_none() {
                        eprintln!(
                            "{} | One of the objects is missing a name property in file {path:?}",
                            "Error".bright_red(),
                        );
                        continue;
                    }

                    let name = data.name.as_ref().unwrap();
                    if name.is_empty() {
                        eprintln!(
                            "{} | One of the object has an empty name propery in file {path:?}",
                            "Error".bright_red(),
                        );
                        continue;
                    }

                    if data.uuid.is_none() {
                        eprintln!(
                            "{} | The object with name {name} doesn't have a uuid in file {path:?}",
                            "Error".bright_red()
                        );
                        continue;
                    }
                    let uuid = data.name.as_ref().unwrap();
                    if uuid.is_empty() {
                        eprintln!(
                            "{} | The object with name {name} has an empty uuid in file {path:?}",
                            "Error".bright_red()
                        );
                        continue;
                    }

                    file_content += &format!("{name} = sm.uuid.new(\"{uuid}\")\n")
                }
            }
        }
    }

    fs::write(UTIL_PATH.to_string() + "/uuids.lua", file_content).unwrap();
}
