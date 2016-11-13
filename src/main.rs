#[macro_use]
extern crate version;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate clap;
#[macro_use]
extern crate error_type;
extern crate data_encoding;
extern crate rand;
extern crate redux;

mod error;
mod generate;
mod text_generator;

use clap::{Arg, App, AppSettings, SubCommand};

fn main() {
    let matches = App::new("Babylon")
        .global_settings(&[AppSettings::ColoredHelp,
                           AppSettings::UnifiedHelpMessage,
                           AppSettings::DeriveDisplayOrder])
        .version(version!())
        .author("Chris Andrews <CodexArcanum@gmail.com>")
        .about("Generate any possible text, randomly")
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("encode")
            .about("Generate a text, incorporating the given initial text")
            .arg(Arg::with_name("TEXT")
                .help("The base text to build from")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("decode")
            .about("Reads an index string and prints the text that it corresponds to")
            .arg(Arg::with_name("INDEX")
                .help("Babylon index string to look up")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("test")
            .about("Encode then decode the text to test the process")
            .arg(Arg::with_name("TEXT")
                .help("The base text to build from")
                .required(true)
                .index(1)))
        .get_matches();

    setup_logging(matches.occurrences_of("v"));

    match matches.subcommand() {
        ("encode", Some(matches)) => {
            let text = matches.value_of("TEXT").unwrap();
            let result = generate::encode(text).unwrap();
            println!("{}", result);
        },
        ("decode", Some(matches)) => {
            let index = matches.value_of("INDEX").unwrap();
            let result = generate::decode(index).unwrap();
            println!("{}", result);
        },
        ("test", Some(matches)) => {
            let index = matches.value_of("TEXT").unwrap();
            let encoded = generate::encode(index).unwrap();
            let decoded = generate::decode(&encoded).unwrap();
            println!("{}|{}", encoded.len(), decoded.len());
        },
        _ => unimplemented!(),
    }
}

fn setup_logging(verbosity: u64) {
    use log::LogLevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::encode::pattern::PatternEncoder;
    use log4rs::config::{Appender, Config, Root};

    let level = match verbosity {
        0 => LogLevelFilter::Off,
        1 => LogLevelFilter::Error,
        2 => LogLevelFilter::Warn,
        3 => LogLevelFilter::Info,
        4 => LogLevelFilter::Debug,
        5 | _ => LogLevelFilter::Trace,
    };

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            //"{d(%Y-%m-%d %H:%M:%S%.6f)(local)} {h({l:<5})} {t} - {m}{n}"
            "{d(%H:%M:%S)(local)} {h({l:<5})} - {m}{n}"
        )))
        .build();

    let root = Root::builder().appender("stdout".to_owned()).build(level);

    let config = Config::builder()
        .appender(Appender::builder().build("stdout".to_owned(), Box::new(stdout)))
        .build(root)
        .unwrap();

    log4rs::init_config(config).unwrap();
}