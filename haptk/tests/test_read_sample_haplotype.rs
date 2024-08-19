mod common;

use crate::common::TEST_VCF;
use std::path::PathBuf;

#[test]
#[cfg(feature = "clap")]
fn read_sample_haplotype() {
    use common::COORDS;
    use haptk::clap::{ClapSelection, ClapStandardArgs};

    let cmd = haptk::clap::SubCommand::Haplotypes {
        args: ClapStandardArgs {
            file: PathBuf::from(TEST_VCF),
            outdir: PathBuf::from("tests/results"),
            coords: String::from(COORDS),
            alleles: ClapSelection::All,
            prefix: None,
            samples: Some(vec![PathBuf::from("tests/data/SAMPLE1.ids")]),
        },
        log_and_verbosity: haptk::clap::LogAndVerbosity {
            verbosity: 1,
            log_file: None,
            silent: false,
        },
    };
    haptk::clap::run_cmd(cmd).unwrap();

    let res = std::fs::read_to_string("tests/results/SAMPLE1_haplotype_1.csv").unwrap();
    insta::assert_yaml_snapshot!(res);

    let res = std::fs::read_to_string("tests/results/SAMPLE1_haplotype_2.csv").unwrap();
    insta::assert_yaml_snapshot!(res);
}
