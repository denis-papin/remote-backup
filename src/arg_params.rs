extern crate clap;
use clap::{Arg, App};

#[derive(Debug)]
pub struct Params {
    pub source_folder : String,
    pub target_url : String,
}

pub fn parse_args() -> Result<Params, Box<dyn std::error::Error>> {

    let matches = App::new("Remote Backup")
        .version("1.0")
        .author("Denis Crespe. <denis@insoftdesign.eu>")
        .about("Remote backups")
        .arg(Arg::with_name("folder")
            .short("f")
            .long("folder")
            .value_name("FOLDER")
            .help("The source folder to scan")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .value_name("URL")
            .help(r#"Complete scp url"# )
            .required(true)
            .takes_value(true))
        .get_matches();


    let folder = matches.value_of("folder").unwrap_or("");
    let url = matches.value_of("url").unwrap_or("");

    let params = Params {
        source_folder : folder.to_string(),
        target_url : url.to_string(),
    };

    dbg!(&params);

    Ok(params)
}
