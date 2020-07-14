extern crate blake2;
extern crate clap;
extern crate csv;
extern crate digest;
extern crate sha1;
extern crate sha2;
extern crate sha3;

use clap::App;
use clap::SubCommand;

mod calc;
mod check;
mod digestutil;
mod error;
mod ioutil;
mod command;

use error::ApplicationError;

fn do_parse<I>(s: &str) -> Result<I, ApplicationError>
where
    I: std::str::FromStr,
{
    match s.parse::<I>() {
        Ok(v) => Ok(v),
        Err(_) => Err(ApplicationError::from_parse_error(s, "failed to parse integer")),
    }
}

fn create_app<'a, 'b>() -> App<'a, 'b> {
    App::new("hast")
        .about("calculate hash")
        .subcommand(
            SubCommand::with_name("calc")
                .about("calc hash")
                .subcommand(command::create_calc_md5())
                .subcommand(command::create_calc_sha1())
                .subcommand(command::create_calc_sha2())
                .subcommand(command::create_calc_sha3())
                .subcommand(command::create_calc_shake())
                .subcommand(command::create_calc_blake2()),
        )
        .subcommand(
            SubCommand::with_name("check")
                .about("check hash")
                .subcommand(command::create_check_md5())
                .subcommand(command::create_check_sha1())
                .subcommand(command::create_check_sha2())
                .subcommand(command::create_check_sha3())
                .subcommand(command::create_check_shake())
                .subcommand(command::create_check_blake2()),
        )
}

fn main() -> Result<(), ApplicationError> {
    let app = create_app();
    let mut app2 = app.clone();
    let matches = app.get_matches();
    match matches.subcommand() {
        ("calc", Some(app)) => match app.subcommand() {
            ("md5", Some(app)) => calc::do_calc_md5(app),
            ("sha1", Some(app)) => calc::do_calc_sha1(app),
            ("sha2", Some(app)) => calc::do_calc_sha2(app),
            ("sha3", Some(app)) => calc::do_calc_sha3(app),
            ("shake", Some(app)) => calc::do_calc_shake(app),
            ("blake2", Some(app)) => calc::do_calc_blake2(app),
            _ => {
                return Err(ApplicationError::from_parameter(
                    "unknown",
                    "unknown command",
                ))
            }
        },
        ("check", Some(app)) => match app.subcommand() {
            ("md5", Some(app)) => check::do_check_md5(app),
            ("sha1", Some(app)) => check::do_check_sha1(app),
            ("sha2", Some(app)) => check::do_check_sha2(app),
            ("sha3", Some(app)) => check::do_check_sha3(app),
            ("shake", Some(app)) => check::do_check_shake(app),
            ("blake2", Some(app)) => check::do_check_blake2(app),
            _ => {
                return Err(ApplicationError::from_parameter(
                    "unknown",
                    "unknown command",
                ))
            }
        },
        _ => {
            match app2.print_long_help() {
                Err(e) => return Err(ApplicationError::Clap(e)),
                _ => (),
            };
            return Err(ApplicationError::from_parameter(
                "unknown",
                "unknown command",
            ));
        }
    }?;
    Ok(())
}
