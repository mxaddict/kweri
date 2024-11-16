use clap::Parser;
use std::fs::{create_dir_all, write};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    query: String,
}

static CONFIG_TOML: &[u8] = include_bytes!("config.toml");

fn main() -> ExitCode {
    let conf_dir = directories::ProjectDirs::from("sh", "kryptic", "kweri").unwrap();
    let conf_path = conf_dir.config_dir().join("config.toml");
    let conf_path_str = conf_path.to_str().unwrap();

    if !conf_path.exists() {
        let res = create_dir_all(conf_dir.config_dir());
        if res.is_err() {
            eprintln!("Can't create config file: {}", conf_path_str);
            return ExitCode::FAILURE;
        }

        let res = write(&conf_path, CONFIG_TOML);

        if res.is_err() {
            eprintln!("Can't create config file: {}", conf_path_str);
            return ExitCode::FAILURE;
        }

        println!("Created default config file: {}", conf_path_str);
    }

    let conf_file = config::File::with_name(conf_path_str);

    let default_conf_str = std::str::from_utf8(CONFIG_TOML).unwrap();
    let default_conf = config::File::from_str(default_conf_str, config::FileFormat::Toml);
    let conf = config::Config::builder()
        .add_source(default_conf)
        .add_source(conf_file)
        .build()
        .unwrap();

    let args = Cli::parse();
    let url = format!("{}{}", conf.get_string("engine").unwrap(), args.query);

    println!("Kwerieng for '{}'", args.query);

    if webbrowser::open(url.as_str()).is_err() {
        eprintln!("Failed to open kweri: '{}'", args.query);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
