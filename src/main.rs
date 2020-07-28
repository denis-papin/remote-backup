
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::{SystemTime,UNIX_EPOCH};
use std::ops::Sub;
use chrono::{DateTime, Local, TimeZone, Duration};
use std::process::exit;

fn most_recent_folder( src_path : &Path ) -> PathBuf {
    let default_sys_time = SystemTime::now().sub(std::time::Duration::from_secs(30*24*60*60) );

    // now - 30 days
    let thirty_days: Duration = Duration::days(30);

    let mut current_date : DateTime<Local> = Local::now() - thirty_days;
    let mut recent_path : PathBuf = Path::new("").to_path_buf();

    for entry in fs::read_dir(src_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let metadata = fs::metadata(&path).unwrap();

        dbg!(&path);

        if metadata.is_dir() {
            let creation_date = metadata.created().unwrap_or(default_sys_time);
            let dt : DateTime<Local>  = system_time_to_date_time(creation_date);

            dbg!(dt, current_date);

            if dt.gt(&current_date) {
                current_date = DateTime::from(dt);
                recent_path = PathBuf::from(path );

                println!("new current date : ");
                dbg!(current_date, &recent_path);
            }
        }
    }

    recent_path
}

/**
*/
fn system_time_to_date_time(t: SystemTime) -> DateTime<Local> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => { // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        },
    };
    Local.timestamp(sec, nsec)
}


fn main() {

    let args: Vec<String> = env::args().collect();
    let mut folder_to_backup : String = String::default();

    if args.len() < 2 {
        println!("{}", show_help());
        exit(45);
    } else {

        let option : &String = &args[1];
        match option.as_ref() {
            "-f" => folder_to_backup = String::from(&args[2] ),
            _ => println!("Wrong argument"),
        }

        if folder_to_backup.is_empty() {
            println!("-f <folder> is required");
            exit(30);
        }
    }


    // Find the most recent folder among the current folders.
    let recent_path = most_recent_folder(&Path::new(&folder_to_backup));

    // Run the scp command for the given folder
    println!("Hello, world! {:?}", &recent_path);
}


/**
    Return the help text.
*/
fn show_help() -> &'static str {

    "
    Remote Backup v0.9.0

    remote-backup -f <folder>

        -f <folder>   Folder to backup

    simple-backup -h

        -h  Show this help file.
"

}