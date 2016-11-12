#[macro_use]
extern crate version;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate clap;

use clap::{Arg, App, AppSettings, SubCommand};

fn main() {
    log4rs::init_file("log4rs.toml", Default::default()).unwrap();

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
        .get_matches();

    match matches.occurrences_of("v") {
        0 => error!("Error level enabled"),
        1 => warn!("Warn level enabled"),
        2 => info!("Info level enabled"),
        3 => debug!("Debug level enabled"),
        4 | _ => trace!("Trace level enabled"),
    }

    match matches.subcommand() {
        ("write", Some(matches)) => {
            let text = matches.value_of("TEXT").unwrap();
            let seed: u64 = matches.value_of("VALUE")
                .and_then(|s| s.parse().ok())
                .unwrap_or(42);
           info!("Generating from '{:?}' with seed={:?}", text, seed);
           // TODO generate some text
        }
        ("lookup", Some(matches)) => {
            let index = matches.value_of("INDEX").unwrap();
            info!("Looking up {:?}", index);
            // TODO lookup some text
        }
        _ => unimplemented!()
    }
}
