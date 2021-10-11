use std::collections::BinaryHeap;
use std::io::{BufRead, Write};

const BEGINNING_DATE: &str = "0000-01-01";

pub struct InfoKeep {
    db: sled::Db,
}

impl InfoKeep {
    pub fn init(path: &str) -> Self {
        Self {
            db: sled::open(path).unwrap(),
        }
    }

    pub fn new_entry(&mut self, key: &str, info: &str) -> String {
        self.db
            .insert(key, info)
            .expect("Could not enter value");
        let mid = self.db.get(key.as_bytes()).unwrap().unwrap();
        let result = String::from_utf8_lossy(&*mid);
        format!("\nInfo added for {}\n{}\n\n", key, result)
    }

    pub fn sort_db(&mut self, sort: bool) -> Vec<(sled::IVec, sled::IVec)> {
        let keys_iter = self.db.iter();
        let mut bh_keys = BinaryHeap::new();

        for k in keys_iter {
            bh_keys.push(k.unwrap());
        }

        let mut keys: Vec<(sled::IVec, sled::IVec)> = bh_keys.into_sorted_vec();

        if sort {
            keys.dedup();

            self.db
                .clear()
                .expect("Could not clear Database of sorting");
            for (k, v) in keys.clone() {
                self.db.insert(k, v).expect("Could not enter value");
            }
        }
        keys
    }

    pub fn export_db(&mut self) {
        let mut file = match std::fs::File::create("ik_Export.text") {
            Ok(file) => file,
            Err(_) => {
                std::fs::remove_file("ik_Export.txt").expect("Could not create file");
                std::fs::File::create("ik_Export.txt").expect("Could not create file")
            }
        };

        let kvs = self.sort_db(false);

        file.write_all("info_keep_file\n".as_bytes())
            .expect("Could not write to export file");
        for (k, v) in kvs {
            let key = String::from_utf8_lossy(&*k);
            let value = String::from_utf8_lossy(&*v);
            file.write_all((key + "=" + value + "\n").as_bytes())
                .expect("Could not write to fill");
        }
    }

    pub fn import_db(&mut self) {
        let file: std::fs::File = std::fs::File::open("ik_Export.txt")
            .expect("No such file called 'ik_Export.txt' found");
        let buf = std::io::BufReader::new(file);
        let mut lines: Vec<String> = buf
            .lines()
            .map(|l| l.expect("could not read file"))
            .collect();
        if lines.contains(&"info_keep_file".to_string()) {
            lines.remove(0);
            for key_value in lines {
                let mid_kv: Vec<&str> = key_value.split('=').collect();
                let k = mid_kv.get(0).unwrap();
                let v = mid_kv.get(1).unwrap();

                let key = k.to_string();
                let value = v.to_string();
                self.db
                    .insert(key.as_bytes(), value.as_bytes())
                    .expect("Could not import data");
            }
        }
        self.print_db();
    }

    pub fn print_db(&self) -> String {
        let keys = self.db.iter();
        let mut output = String::new();
        for (k, v) in keys.flatten() {
            output += &*format!(
                "{} :: {}\n",
                String::from_utf8_lossy(&*k),
                String::from_utf8_lossy(&*v)
            );
        }
        output
    }

    pub fn search_tag(&self, tag: Tag) -> String {
        let full_tag = tag.full_tag();

        let mut output: String;

        if full_tag != BEGINNING_DATE {
            output = format!("Searching for entries with tag: {}\n", full_tag);
        } else {
            output = self.print_db();
            return  "Listing all entries\n".to_owned() + &output;
        };

        let iter: sled::Iter = self.db.range((full_tag.as_bytes())..);

        for i in iter.flatten() {
            let (k, v) = i;

            output += &*format!(
                "{} :: {} \n",
                String::from_utf8_lossy(&*k),
                String::from_utf8_lossy(&*v)
            )
        }

        output
    }

    pub fn remove_info(&mut self, key: &str){
        self.db.remove(key).expect("Could not remove key");
    }

    pub fn clear_db(&mut self){
        self.db.clear().expect("Could not clear Info Keep data");
    }
}

#[derive(Clone, Debug)]
pub struct Tag {
    year: Option<String>,
    month: Option<String>,
    day: Option<String>,
}

impl Tag {
    pub fn new(year: Option<&str>, month: Option<&str>, day: Option<&str>) -> Self {
        // returns None when v.to_string() fails
        Self {
            year: year.map(|v| v.to_string()),
            month: month.map(|v| v.to_string()),
            day: day.map(|v| v.to_string()),
        }
    }
    pub fn full_tag(self) -> String {
        let mut year = "0000".to_string();
        if self.year.is_some() {
            year = self.year.unwrap();
        }

        let mut month = "01".to_string();
        if self.month.is_some() {
            month = self.month.unwrap();
        }

        let mut day = "01".to_string();
        if self.day.is_some() {
            day = self.day.unwrap();
        }

        if year.len() < 4 || year.len() > 4 {
            println!("year should have 4 digits\nFixing\n");
            let mut total_vec = Vec::new();
            for i in year.chars() {
                total_vec.push(i)
            }

            if total_vec.len() < 4 {
                let mut total = "2".to_string();
                for t in total_vec.into_iter() {
                    total += &*char::to_string(&t);
                }
                year = total
            } else {
                println!("Defaulting to 0000");
                year = "0000".to_string()
            }
        }

        if month.len() < 2 || month.len() > 2 {
            println!("year should have 2 digits\nFixing\n");
            let mut total_vec = Vec::new();
            for i in month.chars() {
                total_vec.push(i)
            }

            if total_vec.len() < 4 {
                let mut total = "2".to_string();
                for t in total_vec.into_iter() {
                    total += &*char::to_string(&t);
                }
                month = total
            } else {
                println!("Defaulting to 01");
                month = "01".to_string()
            }
        }

        if day.len() < 2 || day.len() > 2 {
            println!("Day should have 2 digits\nFixing\n");
            let mut total_vec = Vec::new();
            for i in day.chars() {
                total_vec.push(i)
            }

            if total_vec.len() < 4 {
                let mut total = "2".to_string();
                for t in total_vec.into_iter() {
                    total += &*char::to_string(&t);
                }
                day = total
            } else {
                println!("Defaulting to 00 for day\n");
                day = "01".to_string()
            }
        }

        let full_tag: String = year + "-" + &*month + "-" + &*day;

        full_tag
    }
}
