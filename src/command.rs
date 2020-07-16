use clap::App;
use clap::SubCommand;
use clap::{Arg};

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

fn create_output_length_arg<'a, 'b>(default_size: &'a str) -> Arg<'a, 'b> {
    Arg::with_name("outputlength")
        .value_name("OUTPUTLENGTH")
        .default_value(default_size)
        .short("x")
        .long("outputlength")
        .help("hash output length in bytes")
}

pub fn create_calc_sha1<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("sha1")
        .about("calc sha1 hash")
        .arg(create_calc_file_arg())
        .arg(create_output_arg())
}

pub fn create_check_sha1<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("sha1")
        .about("check sha1 hash")
        .arg(create_check_file_arg())
        .arg(create_basepath_arg())
}

pub fn create_calc_sha2<'a, 'b>() -> App<'a, 'b> {
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
                .long("length"),
        )
}

pub fn create_check_sha2<'a, 'b>() -> App<'a, 'b> {
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
                .long("length"),
        )
}

pub fn create_calc_sha3<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("sha3")
        .about("calc sha3 hash")
        .arg(create_calc_file_arg())
        .arg(create_output_arg())
        .arg(create_sha3_length_arg())
}

fn create_sha3_length_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("length")
    .help("bit length")
    .possible_values(&["224", "256", "384", "512"])
    .default_value("256")
    .short("l")
    .long("length")
}

pub fn create_check_sha3<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("sha3")
        .about("check sha3 hash")
        .arg(create_check_file_arg())
        .arg(create_basepath_arg())
        .arg(create_sha3_length_arg())
}

pub fn create_calc_md5<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("md5")
        .about("calc md5 hash")
        .arg(create_calc_file_arg())
        .arg(create_output_arg())
}

pub fn create_check_md5<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("md5")
        .about("check md5 hash")
        .arg(create_check_file_arg())
        .arg(create_basepath_arg())
}

fn create_shake_bitlength_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("length")
    .help("bit length")
    .possible_values(&["128", "256"])
    .default_value("256")
    .short("l")
    .long("length")
}

pub fn create_calc_shake<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("shake")
        .about("calc shake hash")
        .arg(create_calc_file_arg())
        .arg(create_output_arg())
        .arg(create_output_length_arg("128"))
        .arg(create_shake_bitlength_arg())
}

pub fn create_check_shake<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("shake")
        .about("check shake hash")
        .arg(create_check_file_arg())
        .arg(create_output_length_arg("128"))
        .arg(create_shake_bitlength_arg())
}

fn create_blake2_algorithm_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("algorithm")
        .help("blake2 algorithm")
        .possible_values(&["s", "b"])
        .short("a")
        .long("algorithm")
        .default_value("s")
}

pub fn create_calc_blake2<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("blake2")
        .about("calc blake2 hash")
        .arg(create_output_arg())
        .arg(create_blake2_algorithm_arg())
        .arg(create_calc_file_arg())
}

pub fn create_check_blake2<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("blake2")
        .about("check blake2 hash")
        .arg(create_check_file_arg())
        .arg(create_blake2_algorithm_arg())
}