extern crate clap;
extern crate sha1;
extern crate sha2;
extern crate digest;
extern crate csv;

use clap::App;
use clap::Arg;
use clap::SubCommand;
use std::io::Read;

mod error;
mod ioutil;
mod calc;
mod check;

use error::ApplicationError;

fn create_calc_file_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("file")
        .value_name("FILE")
        .default_value("-")
        .multiple(true)
        .required(false)
        .help("input files, '-' means stdin")
}

fn create_check_file_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("file")
        .value_name("FILE")
        .default_value("-")
        .multiple(false)
        .required(false)
        .help("input file(must be output of hast calc result), '-' means stdin")
}

fn create_output_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("output")
        .value_name("OUTPUT_FILE")
        .default_value("-")
        .short("o")
        .long("output")
        .help("output file, '-' means stdout")
}

fn create_basepath_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("basepath")
        .value_name("BASE_PATH")
        .default_value(".")
        .short("b")
        .long("basepath")
        .help("base path for searching file")
}

fn create_app<'a, 'b>() -> App<'a, 'b> {
    App::new("hast")
        .about("calculate hash")
        .subcommand(
            SubCommand::with_name("calc")
            .about("calc hash")
            .subcommand(
                SubCommand::with_name("md5")
                    .about("calc md5 hash")
                    .arg(create_calc_file_arg())
                    .arg(create_output_arg())
            )
            .subcommand(
                SubCommand::with_name("sha1")
                    .about("calc sha1 hash")
                    .arg(create_calc_file_arg())
                    .arg(create_output_arg())
            )
            .subcommand(
                SubCommand::with_name("sha2")
                    .about("calc sha2 hash")
                    .arg(create_calc_file_arg())
                    .arg(create_output_arg())
                    .arg(
                        Arg::with_name("length")
                            .help("bit length")
                            .possible_values(&["224", "256", "384", "512", "512/224", "512/256"])
                            .default_value("256")
                            .short("l")
                            .long("length")
                    )
            )
        )
        .subcommand(
            SubCommand::with_name("check")
            .about("check hash")
            .subcommand(
                SubCommand::with_name("md5")
                    .about("check md5 hash")
                    .arg(create_check_file_arg())
                    .arg(create_basepath_arg())
            )
            .subcommand(
                SubCommand::with_name("sha1")
                    .about("check sha1 hash")
                    .arg(create_check_file_arg())
                    .arg(create_basepath_arg())
            )
            .subcommand(
                SubCommand::with_name("sha2")
                    .about("check sha2 hash")
                    .arg(create_check_file_arg())
                    .arg(create_basepath_arg())
                    .arg(
                        Arg::with_name("length")
                            .help("bit length")
                            .possible_values(&["224", "256", "384", "512", "512/224", "512/256"])
                            .default_value("256")
                            .short("l")
                            .long("length")
                    )
            )
        )
}

fn update_digest<D, R>(d: &mut D, in_f: &mut R) -> Result<(), ApplicationError> where D: digest::Digest, R: Read {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(1024, 0u8);
    loop {
        let n = match in_f.read(&mut buf) {
            Ok(v) => Ok(v),
            Err(e) => Err(ApplicationError::Io(e))
        }?;
        d.update(&buf[0..n]);
        if n == 0 || n < 1024 {
            break;
        }
    }
    Ok(())
}


fn main() -> Result<(), ApplicationError> {
    let app = create_app();
    let mut app2 = app.clone();
    let matches = app.get_matches();
    match matches.subcommand() {
        ("calc", Some(app)) => {
            match app.subcommand() {
                ("md5", Some(app)) => {
                    calc::do_calc_md5(app)
                },
                ("sha1", Some(app)) => {
                    calc::do_calc_sha1(app)
                },
                ("sha2", Some(app)) => {
                    calc::do_calc_sha2(app)
                },
                _ => {
                    return Err(ApplicationError::from_parameter("unknown", "unknown command"))
                }
            }
        },
        ("check", Some(app)) => {
            match app.subcommand() {
                ("md5", Some(app)) => {
                    check::do_check_md5(app)
                },
                ("sha1", Some(app)) => {
                    check::do_check_sha1(app)
                },
                ("sha2", Some(app)) => {
                    check::do_check_sha2(app)
                },
                _ => {
                    return Err(ApplicationError::from_parameter("unknown", "unknown command"))
                }
            }
        },
        _ => {
            match app2.print_long_help() {
                Err(e) => return Err(ApplicationError::Clap(e)),
                _ => ()
            };
            return Err(ApplicationError::from_parameter("unknown", "unknown command"))
        }
    }?;
    Ok(())
}
