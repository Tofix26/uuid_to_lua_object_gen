use std::ops::Index;

use serde::Deserialize;

use crate::utill::FromFile;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub name: String,
    pub uuid: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    pub part_list: Option<Vec<Data>>,
    pub block_list: Option<Vec<Data>>,
    pub scriptable_object_list: Option<Vec<Data>>,
    pub harvestable_list: Option<Vec<Data>>,
    pub tool_list: Option<Vec<Data>>,
    pub characters: Option<Vec<Data>>,
}

impl FromFile<Set> for Set {}

impl Index<&str> for Set {
    type Output = Option<Vec<Data>>;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "part_list" => &self.part_list,
            "block_list" => &self.block_list,
            "scriptable_object_list" => &self.scriptable_object_list,
            "harvestable_list" => &self.harvestable_list,
            "tool_list" => &self.tool_list,
            "characters" => &self.characters,
            _ => &self.part_list,
        }
    }
}
