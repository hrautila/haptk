use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use color_eyre::{
    eyre::{eyre, WrapErr},
    Result,
};
use serde::{Deserialize, Serialize};

use crate::io::{read_lines, return_double_extension_filetype};
use crate::subcommands::bhst::Metadata;

use super::list_markers::HstMetadata;

#[doc(hidden)]
pub fn run(path: PathBuf) -> Result<()> {
    let ids = get_sample_names(path)?;
    for id in ids {
        println!("{id}");
    }
    Ok(())
}

pub fn get_sample_names(path: PathBuf) -> Result<Vec<String>> {
    let extension: &str = Path::new(&path)
        .extension()
        .and_then(OsStr::to_str)
        .ok_or_else(|| eyre!("No filetype in path"))?;

    let extension = match extension {
        "gz" | "bgz" => return_double_extension_filetype(&path, extension)?,
        _ => extension.to_string(),
    };

    let mut ids = vec![];
    match extension.as_str() {
        "vcf.gz" | "vcf" | "bcf" => {
            use rust_htslib::bcf::{Read, Reader};
            let bcf = Reader::from_path(path).expect("Error opening file.");
            let header = bcf.header().clone();
            let samples = header.samples();
            for sample in samples {
                let id = std::str::from_utf8(sample)?;
                ids.push(id.to_string());
            }
        }
        "hst.gz" | "hst" => {
            let samples = read_hst_samples(path)?;
            ids.extend(samples);
        }
        "json.gz" => {
            #[derive(Serialize, Deserialize, Clone)]
            pub struct HstMetadata {
                pub metadata: Metadata,
            }

            let file =
                std::fs::File::open(path.clone()).wrap_err(eyre!("Error opening {path:?}"))?;
            let reader = bgzip::BGZFReader::new(file)?;

            let metadata: HstMetadata = serde_json::from_reader(reader).wrap_err(eyre!(
        "Failed deserializing HSTs from the json.gz. Are you sure the input file is correct?"
    ))?;
            ids.extend(metadata.metadata.samples);
        }
        "fam" => {
            for line in read_lines(path)?.map_while(Result::ok) {
                let mut split = line.split('\t');
                let id = split
                    .nth(1)
                    .expect("error splitting by column, is the file tab delimited?");
                ids.push(id.to_string());
            }
        }
        _ => return Err(eyre!("filetype not supported for: {}", extension)),
    }
    Ok(ids)
}

pub fn read_hst_samples(path: PathBuf) -> Result<Vec<String>> {
    let file = std::fs::File::open(path.clone()).wrap_err(eyre!("Error opening {path:?}"))?;
    let reader = bgzip::BGZFReader::new(file)?;
    let hst: HstMetadata = serde_json::from_reader(reader)?;

    Ok(hst.metadata.samples)
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn test_extension_filetype() {
        let path = std::path::PathBuf::from("test.vcf.gz");
        let ftype = return_double_extension_filetype(&path, "gz").unwrap();
        assert_eq!(String::from("vcf.gz"), ftype);
    }
}
