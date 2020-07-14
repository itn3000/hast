use super::digestutil;
use super::error::ApplicationError;
use super::ioutil;
use clap::ArgMatches;
use digest::Digest;
use std::io::Write;

fn write_calc_result_to_csv_output<W>(
    data: &[u8],
    out_f: &mut csv::Writer<W>,
    inputfile: &str,
    outputfile: &str,
) -> Result<(), ApplicationError>
where
    W: Write,
{
    let mut wstr = String::new();
    for b in data {
        wstr.push_str(format!("{:02x}", b).as_str());
    }
    match out_f.write_record(&[inputfile, wstr.as_str()]) {
        Ok(_) => Ok(()),
        Err(e) => Err(ApplicationError::from_csv(
            e,
            format!("failed to write result({})", outputfile).as_str(),
        )),
    }?;
    Ok(())
}

fn do_calc_fixed_output<D>(
    matches: &ArgMatches,
    mut d: D,
    outputfile: &str,
) -> Result<(), ApplicationError>
where
    D: digest::FixedOutput + digest::Update + digest::Digest,
{
    let out_f = ioutil::create_file_for_write(outputfile)?;
    let mut out_f = csv::Writer::from_writer(out_f);
    if let Some(vals) = matches.values_of("file") {
        for inputfile in vals {
            if inputfile != "-" {
                let globresult = match glob::glob(inputfile) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(ApplicationError::from_glob_pattern_error(
                        e,
                        format!("failed to parse glob({})", inputfile).as_str(),
                    )),
                }?;
                for p1 in globresult {
                    let p = match p1 {
                        Ok(v) => Ok(v),
                        Err(e) => Err(ApplicationError::from_glob_error(
                            e,
                            "failed to get globbed path",
                        )),
                    }?;
                    if let Some(p) = p.to_str() {
                        let mut in_f = ioutil::get_file_or_stdin(p)?;
                        digestutil::update_digest(&mut d, &mut in_f)?;
                        let bytes: Vec<u8> = d.finalize_reset().to_vec();
                        write_calc_result_to_csv_output(&bytes, &mut out_f, p, outputfile)?;
                    } else {
                        return Err(ApplicationError::from_path_error(
                            p.as_path(),
                            "failed to extract path string",
                        ));
                    }
                }
            } else {
                let mut in_f = ioutil::get_file_or_stdin(inputfile)?;
                digestutil::update_digest(&mut d, &mut in_f)?;
                let bytes: Vec<u8> = d.finalize_reset().to_vec();
                write_calc_result_to_csv_output(&bytes, &mut out_f, inputfile, outputfile)?;
            }
        }
    }
    Ok(())
}

pub fn do_calc_sha1(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let outputfile = matches.value_of("output").unwrap_or_else(|| "-");
    do_calc_fixed_output(matches, sha1::Sha1::new(), outputfile)?;
    Ok(())
}

pub fn do_calc_sha2(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let outputfile = matches.value_of("output").unwrap_or_else(|| "-");
    let bitlength = matches.value_of("length").unwrap_or_else(|| "256");
    match bitlength {
        "224" => do_calc_fixed_output(matches, sha2::Sha224::new(), outputfile),
        "256" => do_calc_fixed_output(matches, sha2::Sha256::new(), outputfile),
        "384" => do_calc_fixed_output(matches, sha2::Sha384::new(), outputfile),
        "512" => do_calc_fixed_output(matches, sha2::Sha512::new(), outputfile),
        "512/224" => do_calc_fixed_output(matches, sha2::Sha512Trunc224::new(), outputfile),
        "512/256" => do_calc_fixed_output(matches, sha2::Sha512Trunc256::new(), outputfile),
        _ => Err(ApplicationError::from_parameter(
            "length",
            format!("invalid length parameter({})", bitlength).as_str(),
        )),
    }?;
    Ok(())
}

pub fn do_calc_md5(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let outputfile = matches.value_of("output").unwrap_or_else(|| "-");
    do_calc_fixed_output(matches, md5::Md5::new(), outputfile)?;
    Ok(())
}

pub fn do_calc_sha3(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let outputfile = matches.value_of("output").unwrap_or_else(|| "-");
    let bitlength = matches.value_of("length").unwrap_or_else(|| "256");
    match bitlength {
        "224" => do_calc_fixed_output(matches, sha3::Sha3_224::new(), outputfile),
        "256" => do_calc_fixed_output(matches, sha3::Sha3_256::new(), outputfile),
        "384" => do_calc_fixed_output(matches, sha3::Sha3_384::new(), outputfile),
        "512" => do_calc_fixed_output(matches, sha3::Sha3_512::new(), outputfile),
        _ => Err(ApplicationError::from_parameter(
            "length",
            format!("invalid length parameter({})", bitlength).as_str(),
        )),
    }?;
    Ok(())
}

fn do_calc_extendable_output<D>(
    matches: &ArgMatches,
    mut d: D,
    outputsize: usize,
    outputfile: &str,
) -> Result<(), ApplicationError>
where
    D: digest::ExtendableOutput + digest::Update,
{
    let out_f = ioutil::create_file_for_write(outputfile)?;
    let mut out_f = csv::Writer::from_writer(out_f);
    if let Some(vals) = matches.values_of("file") {
        for inputfile in vals {
            if inputfile != "-" {
                let globresult = match glob::glob(inputfile) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(ApplicationError::from_glob_pattern_error(
                        e,
                        format!("failed to parse glob({})", inputfile).as_str(),
                    )),
                }?;
                for p in globresult {
                    let p = match p {
                        Ok(v) => Ok(v),
                        Err(e) => Err(ApplicationError::from_glob_error(
                            e,
                            "failed to get globbed path",
                        )),
                    }?;
                    if let Some(p) = p.as_path().to_str() {
                        let mut in_f = ioutil::get_file_or_stdin(p)?;
                        digestutil::update_digest(&mut d, &mut in_f)?;
                        let bytes = d.finalize_boxed_reset(outputsize);
                        write_calc_result_to_csv_output(&bytes, &mut out_f, p, outputfile)?;
                    } else {
                        return Err(ApplicationError::from_path_error(
                            p.as_path(),
                            "failed to tostring from path",
                        ));
                    }
                }
            } else {
                let mut in_f = ioutil::get_file_or_stdin(inputfile)?;
                digestutil::update_digest(&mut d, &mut in_f)?;
                let bytes = d.finalize_boxed_reset(outputsize);
                write_calc_result_to_csv_output(&bytes, &mut out_f, inputfile, outputfile)?;
            }
        }
    }
    Ok(())
}

pub fn do_calc_shake(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let outputfile = matches.value_of("output").unwrap_or_else(|| "-");
    let bitlength = matches.value_of("length").unwrap_or_else(|| "128");
    let outlength =
        super::do_parse::<usize>(matches.value_of("outputlength").unwrap_or_else(|| "128"))?;
    match bitlength {
        "128" => {
            do_calc_extendable_output(matches, sha3::Shake128::default(), outlength, outputfile)
        }
        "256" => {
            do_calc_extendable_output(matches, sha3::Shake256::default(), outlength, outputfile)
        }
        _ => Err(ApplicationError::from_parameter(
            "length",
            format!("invalid length parameter({})", bitlength).as_str(),
        )),
    }?;
    Ok(())
}

pub fn do_calc_blake2(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let outputfile = matches.value_of("output").unwrap_or_else(|| "-");
    let algorithm = matches.value_of("algorithm").unwrap_or_else(|| "b");
    match algorithm {
        "b" => do_calc_fixed_output(matches, blake2::Blake2b::new(), outputfile),
        "s" => do_calc_fixed_output(matches, blake2::Blake2s::new(), outputfile),
        _ => Err(ApplicationError::from_parameter(
            "algorithm",
            format!("Unknown algorithm({})", algorithm).as_str(),
        )),
    }?;
    Ok(())
}
