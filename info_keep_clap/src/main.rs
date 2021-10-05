fn main() {
    use chrono::prelude::*;
    use clap::{crate_version, App, Arg, SubCommand};
    use info_keep_lib::{export_db, import_db, new_entry, search_tag, sort_db, Database, Tag};
    use read_input::InputBuild;

    let matches = App::new("Info Keep")
        .version(crate_version!())
        .author("Robert Kimura <chknmanrob1904@gmail.com>")
        .about("Stores information you want to keep using date-time format")
        .arg(
            Arg::with_name("year")
                .short("y")
                .long("year")
                .takes_value(true)
                .help("year for new entry or search")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("month")
                .short("m")
                .long("month")
                .takes_value(true)
                .help("month for new entry or search")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Day")
                .short("d")
                .long("day")
                .takes_value(true)
                .help("Day for new entry or search")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("Adds info to keep: Use parentheses")
                .help("Adds info to keep: info_keep.exe [OPTIONS] new \"<INFO>\"")
                .alias("n")
                .arg(
                    Arg::with_name("INFO")
                        .help("Information you wish to keep")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("search")
                .about("Search kept info, not options for full listing")
                .help("Searches info kept: info_keep.exe [OPTIONS] search")
                .alias("s"),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Deletes a key given date and time key, can use options")
                .alias("d")
                .arg(
                    Arg::with_name("KEY")
                        .help("Key to delete info: 0000-00-00+00:00:00")
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("clear").about("Used to clear info keep"))
        .subcommand(
            SubCommand::with_name("export")
                .about("Exports the Info Keep data to transfer data\n Creates ik_Export.txt")
                .alias("e"),
        )
        .subcommand(
            SubCommand::with_name("import")
                .about("Imports Info Keep data\nReads ik_Export.txt")
                .alias("i"),
        )
        .get_matches();

    let open_db: Database::Db = Database::open("Dates").expect("Error opening Data base");

    let sort = match matches.subcommand().0.is_empty() {
        true => {
            println!("{}", matches.usage());
            false
        }
        false => true,
    };

    let (mut db, _) = sort_db(open_db, sort);

    let key: String;
    let tag: Tag;
    if matches.is_present("year") || matches.is_present("month") || matches.is_present("Day") {
        let year = matches.value_of("year");
        let month = matches.value_of("month");
        let day = matches.value_of("Day");
        let time = Utc::now().format("+%H:%M:%S").to_string();
        tag = Tag::new(year, month, day);
        key = Tag::new(year, month, day).full_tag() + &*time;
    } else {
        tag = Tag::new(None, None, None);
        key = Utc::now().format("%Y-%m-%d+%H:%M:%S").to_string();
    };
    // let key = String::from("Test");

    if let Some(matches) = matches.subcommand_matches("new") {
        let info = matches.value_of("INFO").unwrap();
        db = new_entry(db, &key, info.to_string())
    }
    if matches.subcommand_matches("search").is_some() {
        println!("{}", search_tag(&db, tag));
    }

    if matches.subcommand_matches("import").is_some() {
        db = import_db(db, None);
    }

    if matches.subcommand_matches("export").is_some() {
        export_db(&db);
    }

    if matches.subcommand_matches("clear").is_some() {
        let input = read_input::prelude::input::<bool>()
            .msg("Are you sure (true/False): ")
            .default(false);
        if input.get() {
            db.clear().expect("Could not clear Info Keep data");
        }
    };

    #[cfg(not(debug_assertions))]
    if matches.subcommand_matches("delete").is_some() && matches.is_present("KEY") {
        db.remove(matches.value_of("KEY").unwrap().as_bytes())
            .expect("Could not remove key");
    }

    #[cfg(debug_assertions)]
    db.remove(key.as_bytes()).expect("Key does not exist");
}
