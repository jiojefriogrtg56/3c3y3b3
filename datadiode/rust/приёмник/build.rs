use std::env;
use std::path::PathBuf;

fn main() {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("E:/ruste/recieved/resources/gnfgr5.ico"); // Путь к файлу .ico
        res.compile().unwrap();
    }
}