use crate::Tag;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub trait Database{
    type DBUnit;
    fn init(path: &str) -> Self;
    fn new_entry(&mut self, key: &str, info: &str) -> String;
    fn sort_db(&mut self, sort: bool) -> Vec<(Self::DBUnit, Self::DBUnit)>;
    fn export_db(&mut self);
    fn import_db(&mut self);
    fn print_db(&self) -> String;
    fn search_tag(&self, tag: Tag) -> String;
    fn remove_info(&mut self, key: &str);
    fn clear_db(&mut self);
    fn version() -> String{
        "InfoKeep Library Version: ".to_owned() + VERSION
    }
}