
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::{SystemTime,UNIX_EPOCH};
use std::ops::Sub;
use chrono::{DateTime, Local, TimeZone, Duration};
use obfstr::obfstr;
use subprocess::Exec;

mod arg_params;
use crate::arg_params::{parse_args};

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

        // dbg!(&path);

        if metadata.is_dir() {
            let creation_date = metadata.created().unwrap_or(default_sys_time);
            let dt : DateTime<Local>  = system_time_to_date_time(creation_date);

            //dbg!(dt, current_date);

            if dt.gt(&current_date) {
                current_date = DateTime::from(dt);
                recent_path = PathBuf::from(path );

                //println!("new current date : ");
                //dbg!(current_date, &recent_path);
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



fn main() -> Result<(), Box<dyn std::error::Error>> {

    // TODO Replace this with an cypher obfuscated string, and use a crypto lib to decrypt it.
    let pass = String::from(obfstr!("il faut viser la lune"));

    let params = parse_args()?;

    // Find the most recent folder among the current folders.
    let recent_path = most_recent_folder(&Path::new(&params.source_folder));

    // Run the scp command for the given folder

    let source = recent_path.to_str().expect("Impossible to read the most recent folder");

    println!("Found the most recent folder to backup : {} ", &source);

    println!("Sending to {}", &params.target_url );

    // sshpass -p xxxx scp -r Pictures/ dcrespe@10.42.2.17:/home/dcrespe
    let exit_status = Exec::cmd("sshpass")
        .arg("-p")
        .arg(&pass)
        .arg("scp")
        .arg("-r")
        .arg(source)
        .arg(params.target_url)  // dcrespe@10.42.2.17:/home/dcrespe
        .join();


    println!("Sending Done, status {:?}", &exit_status);

    Ok(())
}
