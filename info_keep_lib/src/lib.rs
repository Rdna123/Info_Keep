use std::collections::BinaryHeap;
use std::io::{BufRead, Write};

pub use sled as Database;



pub fn new_entry(db: Database::Db, key: &str, info: String) -> Database::Db {
    db.insert(key.as_bytes(), info.as_bytes())
        .expect("Could not enter value");
    #[cfg(not(feature = "iced"))]
    {
        println!("\nInfo added for {}", &key);
        let mid = db.get(&key.as_bytes()).unwrap().unwrap();
        let result = String::from_utf8_lossy(&*mid);
        println!("{}\n", result);
    }
    db
}

pub fn export_db(db: &Database::Db) {
    let mut file = match std::fs::File::create("ik_Export.txt") {
        Ok(file) => file,
        Err(_) => {
            std::fs::remove_file("ik_Export.txt").expect("Could not create file");
            std::fs::File::create("ik_Export.txt").expect("Could not create file")
        }
    };
    let (_, kvs) = sort_db(db.clone(), false);
    file.write_all("info_keep_file\n".as_bytes())
        .expect("Could not write to export file");
    for (k, v) in kvs {
        let key = String::from_utf8_lossy(&*k);
        let value = String::from_utf8_lossy(&*v);
        file.write_all((key + "=" + value + "\n").as_bytes())
            .expect("Could not write to fill");
    }
}

pub fn import_db(db: Database::Db, import_file: Option<&Vec<(String, String)>>) -> Database::Db {
    let file: std::fs::File = if import_file.is_none() {
        std::fs::File::open("ik_Export.txt").expect("No such file exists")
    } else {
        let mut file = std::fs::File::create("ik_Export.txt").expect("Could not make data file");
        file.write_all(b"Database_file\n")
            .expect("Could not write to import file");
        for (k, v) in import_file.unwrap() {
            let key = String::from_utf8_lossy(k.as_ref());
            let value = String::from_utf8_lossy(v.as_ref());
            file.write_all((key + "=" + value + "\n").as_bytes())
                .expect("Could not write to fill");
        }
        file
    };
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
            db.insert(key.as_bytes(), value.as_bytes())
                .expect("Could not import data");
        }
    }
    print_db(&db);
    db
}

#[derive(Clone, Debug)]
pub struct Tag {
    year: Option<String>,
    month: Option<String>,
    day: Option<String>,
}

impl Tag {
    pub fn new(year: Option<&str>, month: Option<&str>, day: Option<&str>) -> Self {
        let y = match year {
            None => None,
            _ => Some(year.unwrap().parse::<String>().unwrap()),
        };

        let m = match month {
            None => None,
            _ => Some(month.unwrap().parse::<String>().unwrap()),
        };

        let d = match day {
            None => None,
            _ => Some(day.unwrap().parse::<String>().unwrap()),
        };

        Self {
            year: y,
            month: m,
            day: d,
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

pub fn search_tag(db: &Database::Db, tag: Tag) -> String {
    let full_tag = tag.full_tag();

    if full_tag != "0000-01-01" {
        #[cfg(not(feature = "iced"))]
        println!("Searching for entries with tag: {}", full_tag);
    } else {
        println!("listing all entries");
        print_db(db);
        return "Listing all entries".to_string();
    }

    let iter = db.range((full_tag.as_bytes())..);

    let output = String::new();

    for i in iter.flatten() {
        let (k, v) = i;
        #[cfg(not(feature = "iced"))]
        println!(
            "{} :: {}",
            String::from_utf8_lossy(&*k),
            String::from_utf8_lossy(&*v)
        );

        #[cfg(feature = "iced")]
        {
            output += &*format!(
                "{} :: {} \n",
                String::from_utf8_lossy(&*k),
                String::from_utf8_lossy(&*v)
            )
        }
    }

    output
}

pub fn print_db(db: &Database::Db) {
    let (_, keys) = sort_db(db.clone(), false);
    for (k, v) in keys {
        println!(
            "{} :: {}",
            String::from_utf8_lossy(&*k),
            String::from_utf8_lossy(&*v)
        );
    }
}

pub fn sort_db(db: Database::Db, sort: bool) -> (Database::Db, Vec<(Database::IVec, Database::IVec)>) {
    use std::thread;
    let thread = thread::spawn(move || {
        let keys_iter = db.iter();
        let mut bh_keys = BinaryHeap::new();

        for k in keys_iter {
            bh_keys.push(k.unwrap());
        }

        let mut keys = bh_keys.into_sorted_vec();

        if sort {
            keys.dedup();
            #[cfg(not(feature = "iced"))]
            let time = std::time::Instant::now();

            #[cfg(not(feature = "iced"))]
            println!("###Sorting Keys###");

            db.clear().expect("Could not clear Database of sorting");
            for (k, v) in keys.clone() {
                db.insert(k, v).expect("Could not enter value");
            }

            #[cfg(not(feature = "iced"))]
            println!("DONE: {}s\n", time.elapsed().as_secs());
        }

        (db, keys)
    });
    let result = thread.join();
    result.unwrap()
}
