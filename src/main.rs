#[macro_use] extern crate version;
#[macro_use] extern crate log;
extern crate log4rs;
extern crate clap;
#[macro_use] extern crate error_type;
extern crate lzf;
extern crate data_encoding;
extern crate rand;

mod error;
mod generate;

use clap::{Arg, App, AppSettings, SubCommand};

fn main() {
    let matches = App::new("Babylon")
        .global_settings(&[ AppSettings::ColoredHelp,
                            AppSettings::UnifiedHelpMessage,
                            AppSettings::DeriveDisplayOrder])
        .version(version!())
        .author("Chris Andrews <CodexArcanum@gmail.com>")
        .about("Generate any possible text, randomly")
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("write")
            .about("Generate a text, incorporating the given initial text")
            .arg(Arg::with_name("TEXT")
                .help("The base text to build from")
                .required(true)
                .index(1))
            .arg(Arg::with_name("seed")
                .short("s")
                .long("seed")
                .value_name("SEED")
                .help("Sets a seed value for the RNG")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("lookup")
            .about("Reads an index string and prints the text that it corresponds to")
            .arg(Arg::with_name("INDEX")
                .help("Babylon index string to look up")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("test")
            .about("Generate an index and then look it up again")
            .arg(Arg::with_name("TEXT")
                .help("The base text to build from")
                .required(true)
                .index(1))
            .arg(Arg::with_name("seed")
                .short("s")
                .long("seed")
                .value_name("SEED")
                .help("Sets a seed value for the RNG")
                .takes_value(true)))
        .get_matches();

    setup_logging(matches.occurrences_of("v"));

    match matches.subcommand() {
        ("write", Some(matches)) => {
            let text = matches.value_of("TEXT").unwrap();
            let seed: u64 = matches.value_of("VALUE")
                .and_then(|s| s.parse().ok())
                .unwrap_or(42);
           let result = generate::write(text).unwrap();
           println!("{}", result);
        },
        ("lookup", Some(matches)) => {
            let index = matches.value_of("INDEX").unwrap();
            let result = generate::lookup(index).unwrap();
            println!("{}", result);
        },
        ("test", Some(matches)) => {
            let text = matches.value_of("TEXT").unwrap();
            let seed: u64 = matches.value_of("VALUE")
                .and_then(|s| s.parse().ok())
                .unwrap_or(42);
           let index = generate::write(text).unwrap();
           let result = generate::lookup(&index).unwrap();
           println!("{}", result);
        },
        _ => unimplemented!()
    }
}

fn setup_logging(verbosity:u64) {
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
            "{d(%Y-%m-%d %H:%M:%S%.6f)(local)} {h({l:<5})} {t} - {m}{n}")))
        .build();

    let root = Root::builder().appender("stdout".to_owned()).build(level);

    let config = Config::builder()
        .appender(Appender::builder().build("stdout".to_owned(), Box::new(stdout)))
        .build(root)
        .unwrap();

    log4rs::init_config(config).unwrap();
}