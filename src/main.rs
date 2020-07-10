extern crate clap;
extern crate sha1;
extern crate sha2;
extern crate digest;
extern crate csv;

use clap::App;
use clap::Arg;
use clap::SubCommand;
use clap::ArgMatches;
use std::io::Read;
use std::io::Write;
use std::io::Error as IoError;
use digest::Digest;

#[derive(Debug)]
struct InvalidParameter {
    name: String,
    message: String
}

#[derive(Debug)]
struct CheckError {
    message: String,
    filename1: String,
    filename2: String,
    hash1: String,
    hash2: String
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(file1 = ({}, {}), file2 = ({}, {})", self.message, self.filename1, self.hash1, self.filename2, self.hash2)
    }
}

#[derive(Debug)]
struct CsvError {
    message: String,
    kind: csv::ErrorKind
}
#[derive(Debug)]
enum ApplicationError {
    Io(std::io::Error),
    Parameter(InvalidParameter),
    Clap(clap::Error),
    Check(CheckError),
    Csv(CsvError)
}

impl ApplicationError {
    pub fn from_io(e: &IoError, msg: &str) -> ApplicationError {
        ApplicationError::Io(IoError::new(e.kind(), format!("{}: {}", msg, e.to_string())))
    }
    pub fn from_parameter(name: &str, msg: &str) -> ApplicationError {
        ApplicationError::Parameter(InvalidParameter {
            name: name.to_string(),
            message: msg.to_string()
        })
    }
    pub fn from_check(message: &str, f1: &str, f2: &str, h1: &str, h2: &str) -> ApplicationError {
        ApplicationError::Check(CheckError {
            message: message.to_owned(),
            filename1: f1.to_owned(),
            filename2: f2.to_owned(),
            hash1: h1.to_owned(),
            hash2: h2.to_owned()
        })
    }
    pub fn from_csv(e: csv::Error, msg: &str) -> ApplicationError {
        ApplicationError::Csv(CsvError {
            message: msg.to_owned(),
            kind: e.into_kind()
        })
    }
}

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

fn get_file_or_stdin(filepath: &str) -> Result<Box<dyn Read>, ApplicationError> {
    if filepath != "-" {
        match std::fs::File::open(filepath) {
            Ok(v) => Ok(Box::new(v)),
            Err(e) => Err(ApplicationError::from_io(&e, format!("failed to open file for read({})", filepath).as_str()))
        }
    } else {
        Ok(Box::new(std::io::stdin()))
    }
}

fn create_file_for_write(path: &str) -> Result<Box<dyn Write>, ApplicationError> {
    if path != "-" {
        match std::fs::File::create(path) {
            Ok(v) => Ok(Box::new(v)),
            Err(e) => Err(ApplicationError::from_io(&e, format!("failed to create file for write({})", path).as_str()))
        }
    } else {
        Ok(Box::new(std::io::stdout()))
    }
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

fn write_calc_result_to_csv_output<W>(data: &[u8], out_f: &mut csv::Writer<W>, inputfile: &str, outputfile: &str) -> Result<(), ApplicationError> where W: Write {
    let mut wstr = String::new();
    for b in data {
        wstr.push_str(format!("{:x}", b).as_str());
    }
    match out_f.write_record(&[inputfile, wstr.as_str()]) {
        Ok(_) => Ok(()),
        Err(e) => Err(ApplicationError::from_csv(e, format!("failed to write result({})", outputfile).as_str()))
    }?;
    Ok(())
}

fn do_calc_sha1(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let outputfile = matches.value_of("output").unwrap_or_else(|| "-");
    let out_f = create_file_for_write(outputfile)?;
    let mut out_f = csv::Writer::from_writer(out_f);
    if let Some(vals) = matches.values_of("file") {
        for inputfile in vals {
            let mut in_f = get_file_or_stdin(inputfile)?;
            let mut d: sha1::Sha1 = sha1::Sha1::default();
            update_digest(&mut d, &mut in_f)?;
            let data: Vec<u8> = d.finalize().to_vec();
            write_calc_result_to_csv_output(&data, &mut out_f, inputfile, outputfile)?;
        }
    }
    Ok(())
}

fn do_calc_sha2_process<D>(matches: &ArgMatches, mut d: D, outputfile: &str) -> Result<(), ApplicationError> where D: sha2::Digest {
    let out_f = create_file_for_write(outputfile)?;
    let mut out_f = csv::Writer::from_writer(out_f);
    if let Some(vals) = matches.values_of("file") {
        for inputfile in vals {
            let mut in_f = get_file_or_stdin(inputfile)?;
            update_digest(&mut d, &mut in_f)?;
            let bytes: Vec<u8> = d.finalize_reset().to_vec();
            write_calc_result_to_csv_output(&bytes, &mut out_f, inputfile, outputfile)?;
        }
    }
    Ok(())
}

fn do_calc_sha2(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let outputfile = matches.value_of("output").unwrap_or_else(|| "-");
    let bitlength = matches.value_of("length").unwrap_or_else(|| "256");
    match bitlength {
        "224" => do_calc_sha2_process(matches, sha2::Sha224::new(), outputfile),
        "256" => do_calc_sha2_process(matches, sha2::Sha256::new(), outputfile),
        "384" => do_calc_sha2_process(matches, sha2::Sha384::new(), outputfile),
        "512" => do_calc_sha2_process(matches, sha2::Sha512::new(), outputfile),
        "512/224" => do_calc_sha2_process(matches, sha2::Sha512Trunc224::new(), outputfile),
        "512/256" => do_calc_sha2_process(matches, sha2::Sha512Trunc256::new(), outputfile),
        _ => Err(ApplicationError::from_parameter("length", format!("invalid length parameter({})", bitlength).as_str()))
    }?;
    Ok(())
}

fn check_hash<D>(basepath: &str, inputfile: &str, expected_hash: &str, d: &mut D) -> Result<(), ApplicationError> where D: digest::Digest {
    let filepath = if inputfile == "-" {
        "-".to_owned()
    } else {
        let mut p = std::path::PathBuf::new();
        p.push(basepath);
        p.push(inputfile);
        match p.to_str() {
            Some(v) => v.to_owned(),
            None => return Err(ApplicationError::from_parameter("filename", format!("filename combine error({}, {})", basepath, inputfile).as_str()))
        }.to_owned()
    };
    let mut in_f = get_file_or_stdin(filepath.as_str())?;
    update_digest(d, &mut in_f)?;
    let hash = d.finalize_reset();
    let mut hashstr = String::new();
    for b in hash {
        hashstr.push_str(format!("{:x}", b).as_str());
    }
    if expected_hash != hashstr {
        return Err(ApplicationError::from_check("hash check failed", &inputfile, &filepath, &expected_hash, &hashstr));
    }
    Ok(())
}

fn do_check_hash_from_csv<D>(mut d: D, inputfile: &str, basepath: &str) -> Result<(), ApplicationError> where D: digest::Digest {
    let in_f = get_file_or_stdin(inputfile)?;
    let mut in_f = csv::Reader::from_reader(in_f);
    for record in in_f.records() {
        match record {
            Ok(v) => {
                if v.len() >= 2 {
                    check_hash(basepath, &v[0], &v[1], &mut d)
                } else {
                    Ok(())
                }
            },
            Err(e) => {
                Err(ApplicationError::from_csv(e, format!("failed to read csv file({})", inputfile).as_str()))
            }
        }?;
    }
    Ok(())
}

fn do_check_sha1(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let inputfile = matches.value_of("file").unwrap_or_else(|| "-");
    let basepath = matches.value_of("basepath").unwrap_or_else(|| ".");
    let d = sha1::Sha1::new();
    do_check_hash_from_csv(d, inputfile, basepath)?;
    Ok(())
}

fn do_check_sha2(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let inputfile = matches.value_of("file").unwrap_or_else(|| "-");
    let basepath = matches.value_of("basepath").unwrap_or_else(|| ".");
    let bitlength = matches.value_of("length").unwrap_or_else(|| "256");
    match bitlength {
        "224" => do_check_hash_from_csv(sha2::Sha224::new(), inputfile, basepath),
        "256" => do_check_hash_from_csv(sha2::Sha256::new(), inputfile, basepath),
        "384" => do_check_hash_from_csv(sha2::Sha384::new(), inputfile, basepath),
        "512" => do_check_hash_from_csv(sha2::Sha512::new(), inputfile, basepath),
        "512/224" => do_check_hash_from_csv(sha2::Sha512Trunc224::new(), inputfile, basepath),
        "512/256" => do_check_hash_from_csv(sha2::Sha512Trunc256::new(), inputfile, basepath),
        _ => Err(ApplicationError::from_parameter("length", format!("invalid length parameter({})", bitlength).as_str()))
    }?;
    Ok(())
}

fn do_calc_md5(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let outputfile = matches.value_of("output").unwrap_or_else(|| "-");
    let out_f = create_file_for_write(outputfile)?;
    let mut out_f = csv::Writer::from_writer(out_f);
    if let Some(vals) = matches.values_of("file") {
        for inputfile in vals {
            let mut in_f = get_file_or_stdin(inputfile)?;
            let mut d = md5::Md5::new();
            update_digest(&mut d, &mut in_f)?;
            let data: Vec<u8> = d.finalize().to_vec();
            write_calc_result_to_csv_output(&data, &mut out_f, inputfile, outputfile)?;
        }
    }
    Ok(())
}

fn do_check_md5(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let inputfile = matches.value_of("file").unwrap_or_else(|| "-");
    let basepath = matches.value_of("basepath").unwrap_or_else(|| ".");
    let d = md5::Md5::new();
    do_check_hash_from_csv(d, inputfile, basepath)?;
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
                    do_calc_md5(app)
                },
                ("sha1", Some(app)) => {
                    do_calc_sha1(app)
                },
                ("sha2", Some(app)) => {
                    do_calc_sha2(app)
                },
                _ => {
                    return Err(ApplicationError::from_parameter("unknown", "unknown command"))
                }
            }
        },
        ("check", Some(app)) => {
            match app.subcommand() {
                ("md5", Some(app)) => {
                    do_check_md5(app)
                },
                ("sha1", Some(app)) => {
                    do_check_sha1(app)
                },
                ("sha2", Some(app)) => {
                    do_check_sha2(app)
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
