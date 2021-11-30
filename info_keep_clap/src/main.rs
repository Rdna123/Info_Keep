use clap::{crate_version, App, Arg, SubCommand};
use info_keep_lib::{InfoKeep, Tag, Time};
use read_input::InputBuild;
use info_keep_lib::database::Database;

fn main() {
    let matches = App::new("InfoKeep Clap")
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
        .arg(
            Arg::with_name("Version")
                .short("v")
                .long("version")
                .takes_value(false)
                .help("prints version")
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
                .about("Search kept info, use no Tag args for full listing")
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
                        .takes_value(true)
                        .required(true),
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

    if matches.is_present("Version"){
        println!("InfoKeep Clap Version: {}", crate_version!());
        println!("{}", InfoKeep::version());
        return;
    }

    let sort = match matches.subcommand().0.is_empty() {
        true => {
            println!("{}", matches.usage());
            return;
        }
        false => true,
    };

    let mut db: InfoKeep = InfoKeep::init("Dates");


    //let (mut db, _) = sort_db(open_db, sort);
    db.sort_db(sort);

    let key: String;
    let tag: Tag;
    if matches.is_present("year") || matches.is_present("month") || matches.is_present("Day") {
        let year = matches.value_of("year");
        let month = matches.value_of("month");
        let day = matches.value_of("Day");
        let time = Time::generate_time();
        tag = Tag::new(year, month, day);
        key = Tag::new(year, month, day).full_tag() + &*time;
    } else {
        tag = Tag::new(None, None, None);
        key = Time::generate_timestamp();
    };
    // let key = String::from("Test");

    if let Some(m) = matches.subcommand_matches("new") {
        let info = m.value_of("INFO").unwrap();
        db.new_entry(&key, info);
    }
    if matches.subcommand_matches("search").is_some() {
        println!("{}", db.search_tag(tag));
    }

    if matches.subcommand_matches("import").is_some() {
        db.import_db();
    }

    if matches.subcommand_matches("export").is_some() {
        db.export_db();
    }

    if matches.subcommand_matches("clear").is_some() {
        let input = read_input::prelude::input::<bool>()
            .msg("Are you sure (true/False): ")
            .default(false);
        if input.get() {
            db.clear_db();
        }
    };

    #[cfg(not(debug_assertions))]
    if let Some(m) = matches.subcommand_matches("delete") {
        println!("Removed info from {}", m.value_of("KEY").unwrap());
        db.remove_info(m.value_of("KEY").unwrap());
    }

    #[cfg(debug_assertions)]
    db.remove_info(&key);
}
