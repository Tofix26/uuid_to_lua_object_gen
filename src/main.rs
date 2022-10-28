mod errors;
mod structs;
mod utill;

use crate::utill::FromFile;
use clap::Parser;
use error_stack::{IntoReport, Result, ResultExt};
use errors::{FolderGenError, SetGenError};
use log::{error, info, warn};
use owo_colors::OwoColorize;
use std::{
    fs::{self, create_dir_all, File},
    io::Write,
    path::Path,
    process,
};
use structs::{arguments::Args, database::Db, misc::Folder, set::Set};

const FOLDERS: [Folder; 5] = [
    Folder {
        path: "./Objects/Database/",
        file: "shapesets.shapedb",
        entries: ["part_list", "block_list"],
        set_list: "shape_set_list",
        set_entry: None,
    },
    Folder {
        path: "./ScriptableObjects/",
        file: "scriptableObjectSets.sobdb",
        entries: ["scriptable_object_list", ""],
        set_list: "scriptable_object_set_list",
        set_entry: Some("scriptable_object_set"),
    },
    Folder {
        path: "./Harvestables/Database/",
        file: "harvestablesets.harvestabledb",
        entries: ["harvestable_list", ""],
        set_list: "harvestable_set_list",
        set_entry: Some("harvestable_object_set"),
    },
    Folder {
        path: "./Tools/Database/",
        file: "toolsets.tooldb",
        entries: ["tool_list", ""],
        set_list: "tool_set_list",
        set_entry: None,
    },
    Folder {
        path: "./Characters/Database/",
        file: "charactersets.characterdb",
        entries: ["characters", ""],
        set_list: "character_set_list",
        set_entry: None,
    },
];

fn main() {
    simple_logger::SimpleLogger::new()
        .init()
        .expect("Failed to init a logger");

    let config = fs::read_to_string(".sm_uid_gen");

    let args = match config {
        Ok(str) => serde_json::from_str(&str).unwrap_or_else(|err| {
            error!("Failed to parse config file {err}");
            Args::parse()
        }),
        Err(_) => Args::parse(),
    };

    let script_name = args.file_name;
    let output_dir = args.output_dir;

    let mut file_content =
        String::from("---@diagnostic disable: lowercase-global\n-- this file is generated\n");

    create_dir_all(output_dir.clone()).unwrap_or_else(|_| {
        println!(
            "{} | No permission to create {}",
            "Error".bright_red(),
            output_dir
        );
        process::exit(1)
    });

    let mut file = File::create(&format!("{output_dir}/{script_name}")).unwrap_or_else(|_| {
        println!(
            "{} | No permission to create {}",
            "Error".bright_red(),
            script_name
        );
        process::exit(1)
    });

    for folder in FOLDERS.iter() {
        if !Path::new(&format!("{}{}", folder.path, folder.file)).exists() {
            continue;
        }

        match gen_folder(folder) {
            Ok(str) => {
                info!("Generated {}{} succesfully", folder.path, folder.file);
                file_content += &str
            }
            Err(_) => {
                error!("Failed to generate {}{}", folder.path, folder.file)
            }
        }
    }

    match file.write_all(file_content.as_bytes()) {
        Ok(_) => {}
        Err(_) => {
            error!("Failed to write to file {output_dir}/{script_name}")
        }
    }
}

fn gen_folder(folder: &Folder) -> Result<String, FolderGenError> {
    let mut file_content = String::new();

    let db = Db::from_file(&format!("{}{}", folder.path, folder.file))
        .change_context(FolderGenError)
        .attach_printable(format!("Failed to parse db {}{}", folder.path, folder.file))?;

    let data = db[folder.set_list]
        .as_ref()
        .ok_or(FolderGenError)
        .into_report()
        .attach_printable_lazy(|| {
            format!(
                "Failed to get data out of database {}{}",
                folder.path, folder.file
            )
        })?;

    let data = data
        .get_vec(folder.set_entry)
        .ok_or(FolderGenError)
        .into_report()
        .attach_printable_lazy(|| {
            format!(
                "Failed to get data out of database {}{}",
                folder.path, folder.file
            )
        })?;

    for path in data {
        match gen_set(path.replace("$CONTENT_DATA", "."), folder) {
            Ok(str) => {
                file_content += &str;
            }
            Err(_) => {}
        }
    }

    Ok(file_content)
}

fn gen_set(path: String, folder: &Folder) -> Result<String, SetGenError> {
    let mut file_content = String::new();
    let set = Set::from_file(&path)
        .change_context(SetGenError)
        .attach_printable_lazy(|| format!("Failed to parse set {path}"))?;

    for entry in folder.entries {
        if entry.is_empty() {
            continue;
        }

        let data = &set[entry];

        if let Some(vec) = data {
            if !vec.is_empty() {
                file_content += &format!("\n----------------------------------------\n--{path}\n----------------------------------------\n\n");
            }

            for data in vec {
                if data.name.contains(" ") {
                    warn!(
                        "{} has an space in its name replacing with \"_\"... ({path})",
                        data.name
                    )
                }

                file_content += &format!(
                    "{} = sm.uuid.new(\"{}\")\n",
                    data.name.replace(" ", "_"),
                    data.uuid
                );
            }
        }
    }

    Ok(file_content)
}
