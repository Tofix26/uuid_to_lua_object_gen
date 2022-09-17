use serde::{Deserialize, Serialize};

pub struct Folder {
    pub path: &'static str,
    pub file: &'static str,
    pub entries: [&'static str; 2],
    pub set_list: &'static str,
    pub set_entry: Option<&'static str>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Databse {
    pub shapeSetList: Option<DbType>,
    pub harvestableSetList: Option<DbType>,
    pub toolSetList: Option<DbType>,
    pub scriptableObjectSetList: Option<DbType>,
    pub characterSetList: Option<DbType>,
}

impl Databse {
    pub fn index(&self, index: &str) -> &Option<DbType> {
        match index {
            "shapeSetList" => &self.shapeSetList,
            "harvestableSetList" => &self.harvestableSetList,
            "toolSetList" => &self.toolSetList,
            "scriptableObjectSetList" => &self.scriptableObjectSetList,
            "characterSetList" => &self.characterSetList,
            _ => &self.shapeSetList,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Set {
    pub partList: Option<Vec<Data>>,
    pub blockList: Option<Vec<Data>>,
    pub scriptableObjectList: Option<Vec<Data>>,
    pub harvestableList: Option<Vec<Data>>,
    pub toolList: Option<Vec<Data>>,
    pub characters: Option<Vec<Data>>,
}

impl Set {
    pub fn index(&self, index: &str) -> &Option<Vec<Data>> {
        match index {
            "blockList" => &self.blockList,
            "scriptableObjectList" => &self.scriptableObjectList,
            "harvestableList" => &self.harvestableList,
            "toolList" => &self.toolList,
            "characters" => &self.characters,
            _ => &self.partList,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub uuid: Option<String>,
    pub name: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Entries {
    scriptableObjectSet: Option<String>,
    name: Option<String>,
}

impl Entries {
    pub fn parse_entries(vec: &Vec<Entries>, key: &str) -> Vec<String> {
        vec.iter()
            .map(|entry| match key {
                "name" => entry.name.clone().unwrap(),
                "scriptableObjectSet" => entry.scriptableObjectSet.clone().unwrap(),
                _ => entry.scriptableObjectSet.clone().unwrap(),
            })
            .collect::<Vec<String>>()
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum DbType {
    Entry(Vec<Entries>),
    Data(Vec<String>),
}
