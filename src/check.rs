use super::ioutil;
use super::error::ApplicationError;
use digest::Digest;
use clap::ArgMatches;
use super::digestutil;

fn check_hash_fixed<D>(basepath: &str, inputfile: &str, expected_hash: &str, d: &mut D) -> Result<(), ApplicationError> where D: digest::Digest + digest::Update {
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
    let mut in_f = ioutil::get_file_or_stdin(filepath.as_str())?;
    digestutil::update_digest(d, &mut in_f)?;
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

fn do_check_hash_fixed_from_csv<D>(mut d: D, inputfile: &str, basepath: &str) -> Result<(), ApplicationError> where D: digest::Digest + digest::Update {
    let in_f = ioutil::get_file_or_stdin(inputfile)?;
    let mut in_f = csv::Reader::from_reader(in_f);
    for record in in_f.records() {
        match record {
            Ok(v) => {
                if v.len() >= 2 {
                    check_hash_fixed(basepath, &v[0], &v[1], &mut d)
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

pub fn do_check_sha1(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let inputfile = matches.value_of("file").unwrap_or_else(|| "-");
    let basepath = matches.value_of("basepath").unwrap_or_else(|| ".");
    let d = sha1::Sha1::new();
    do_check_hash_fixed_from_csv(d, inputfile, basepath)?;
    Ok(())
}

pub fn do_check_sha2(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let inputfile = matches.value_of("file").unwrap_or_else(|| "-");
    let basepath = matches.value_of("basepath").unwrap_or_else(|| ".");
    let bitlength = matches.value_of("length").unwrap_or_else(|| "256");
    match bitlength {
        "224" => do_check_hash_fixed_from_csv(sha2::Sha224::new(), inputfile, basepath),
        "256" => do_check_hash_fixed_from_csv(sha2::Sha256::new(), inputfile, basepath),
        "384" => do_check_hash_fixed_from_csv(sha2::Sha384::new(), inputfile, basepath),
        "512" => do_check_hash_fixed_from_csv(sha2::Sha512::new(), inputfile, basepath),
        "512/224" => do_check_hash_fixed_from_csv(sha2::Sha512Trunc224::new(), inputfile, basepath),
        "512/256" => do_check_hash_fixed_from_csv(sha2::Sha512Trunc256::new(), inputfile, basepath),
        _ => Err(ApplicationError::from_parameter("length", format!("invalid length parameter({})", bitlength).as_str()))
    }?;
    Ok(())
}

pub fn do_check_md5(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let inputfile = matches.value_of("file").unwrap_or_else(|| "-");
    let basepath = matches.value_of("basepath").unwrap_or_else(|| ".");
    let d = md5::Md5::new();
    do_check_hash_fixed_from_csv(d, inputfile, basepath)?;
    Ok(())
}

pub fn do_check_sha3(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let inputfile = matches.value_of("file").unwrap_or_else(|| "-");
    let basepath = matches.value_of("basepath").unwrap_or_else(|| ".");
    let bitlength = matches.value_of("length").unwrap_or_else(|| "256");
    match bitlength {
        "224" => do_check_hash_fixed_from_csv(sha3::Sha3_224::new(), inputfile, basepath),
        "256" => do_check_hash_fixed_from_csv(sha3::Sha3_256::new(), inputfile, basepath),
        "384" => do_check_hash_fixed_from_csv(sha3::Sha3_384::new(), inputfile, basepath),
        "512" => do_check_hash_fixed_from_csv(sha3::Sha3_512::new(), inputfile, basepath),
        _ => Err(ApplicationError::from_parameter("length", format!("invalid length parameter({})", bitlength).as_str()))
    }?;
    Ok(())
}

fn check_hash_extendable<D>(d: &mut D, basepath: &str, inputfile: &str, outputlength: usize, expected_hash: &str) -> Result<(), ApplicationError> where D: digest::ExtendableOutput + digest::Update{
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
    let mut in_f = ioutil::get_file_or_stdin(filepath.as_str())?;
    digestutil::update_digest(d, &mut in_f)?;
    let hash = d.finalize_boxed_reset(outputlength);
    let mut hashstr = String::new();
    for b in hash.into_iter() {
        hashstr.push_str(format!("{:x}", b).as_str());
    }
    if expected_hash != hashstr {
        return Err(ApplicationError::from_check("hash check failed", &inputfile, &filepath, &expected_hash, &hashstr));
    }
    Ok(())
}

fn do_check_hash_extendable_from_csv<D>(mut d: D, inputfile: &str, basepath: &str, outputlength: usize) -> Result<(), ApplicationError> where D: digest::ExtendableOutput + digest::Update {
    let in_f = ioutil::get_file_or_stdin(inputfile)?;
    let mut in_f = csv::Reader::from_reader(in_f);
    for record in in_f.records() {
        match record {
            Ok(v) => {
                if v.len() >= 2 {
                    check_hash_extendable(&mut d, basepath, &v[0], outputlength, &v[1], )
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

pub fn do_check_shake(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let inputfile = matches.value_of("file").unwrap_or_else(|| "-");
    let basepath = matches.value_of("basepath").unwrap_or_else(|| ".");
    let bitlength = matches.value_of("length").unwrap_or_else(|| "128");
    let outlength = super::do_parse::<usize>(matches.value_of("outputlength").unwrap_or_else(|| "128"))?;
    match bitlength {
        "128" => do_check_hash_extendable_from_csv(sha3::Shake128::default(), inputfile, basepath, outlength),
        "256" => do_check_hash_extendable_from_csv(sha3::Shake256::default(), inputfile, basepath, outlength),
        _ => Err(ApplicationError::from_parameter(
            "length",
            format!("invalid length parameter({})", bitlength).as_str(),
        )),
    }?;
    Ok(())
}

pub fn do_check_blake2(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let inputfile = matches.value_of("file").unwrap_or_else(|| "-");
    let basepath = matches.value_of("basepath").unwrap_or_else(|| ".");
    let algorithm = matches.value_of("algorithm").unwrap_or_else(|| "b");
    match algorithm {
        "b" => do_check_hash_fixed_from_csv(blake2::Blake2b::new(), inputfile, basepath),
        "s" => do_check_hash_fixed_from_csv(blake2::Blake2s::new(), inputfile, basepath),
        _ => Err(ApplicationError::from_parameter("algorithm", format!("Unknown algorithm({})", algorithm).as_str()))
    }?;
    Ok(())
}