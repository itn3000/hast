use clap::ArgMatches;
use super::error::ApplicationError;
use super::ioutil;
use std::io::Write;
use digest::Digest;

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

pub fn do_calc_sha1(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let outputfile = matches.value_of("output").unwrap_or_else(|| "-");
    let out_f = ioutil::create_file_for_write(outputfile)?;
    let mut out_f = csv::Writer::from_writer(out_f);
    if let Some(vals) = matches.values_of("file") {
        for inputfile in vals {
            let mut in_f = ioutil::get_file_or_stdin(inputfile)?;
            let mut d: sha1::Sha1 = sha1::Sha1::default();
            super::update_digest(&mut d, &mut in_f)?;
            let data: Vec<u8> = d.finalize().to_vec();
            write_calc_result_to_csv_output(&data, &mut out_f, inputfile, outputfile)?;
        }
    }
    Ok(())
}

fn do_calc_sha2_process<D>(matches: &ArgMatches, mut d: D, outputfile: &str) -> Result<(), ApplicationError> where D: sha2::Digest {
    let out_f = ioutil::create_file_for_write(outputfile)?;
    let mut out_f = csv::Writer::from_writer(out_f);
    if let Some(vals) = matches.values_of("file") {
        for inputfile in vals {
            let mut in_f = ioutil::get_file_or_stdin(inputfile)?;
            super::update_digest(&mut d, &mut in_f)?;
            let bytes: Vec<u8> = d.finalize_reset().to_vec();
            write_calc_result_to_csv_output(&bytes, &mut out_f, inputfile, outputfile)?;
        }
    }
    Ok(())
}

pub fn do_calc_sha2(matches: &ArgMatches) -> Result<(), ApplicationError> {
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


pub fn do_calc_md5(matches: &ArgMatches) -> Result<(), ApplicationError> {
    let outputfile = matches.value_of("output").unwrap_or_else(|| "-");
    let out_f = ioutil::create_file_for_write(outputfile)?;
    let mut out_f = csv::Writer::from_writer(out_f);
    if let Some(vals) = matches.values_of("file") {
        for inputfile in vals {
            let mut in_f = ioutil::get_file_or_stdin(inputfile)?;
            let mut d = md5::Md5::new();
            super::update_digest(&mut d, &mut in_f)?;
            let data: Vec<u8> = d.finalize().to_vec();
            write_calc_result_to_csv_output(&data, &mut out_f, inputfile, outputfile)?;
        }
    }
    Ok(())
}

