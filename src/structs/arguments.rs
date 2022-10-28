use clap::Parser;

///Simple program to generate an uuid database from all stuff in sm
#[derive(Parser, Debug)]
pub struct Args {
    ///The folder path you want to put the file in **DONT** end this with an /. Default = "./Scripts/util"
    #[arg(short, long, default_value = "./Scripts/util")]
    pub output_dir: String,

    ///The name of the file that will be outputed. Default = "uuids.lua"
    #[arg(short, long, default_value = "uuids.lua")]
    pub file_name: String,
}
