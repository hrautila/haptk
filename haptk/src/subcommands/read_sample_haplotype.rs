use color_eyre::{eyre::ensure, Result};

use crate::{
    args::StandardArgs,
    core::{open_csv_writer, parse_coords},
    read_vcf::read_vcf_to_matrix,
    structs::HapVariant,
    utils::push_to_output,
};

#[doc(hidden)]
#[tracing::instrument]
pub fn run(args: StandardArgs) -> Result<()> {
    let (contig, start, stop) = parse_coords(&args.coords)?;

    let vcf = match (start, stop) {
        (Some(start), Some(stop)) => {
            read_vcf_to_matrix(&args, contig, 0, Some((start, stop)), None)?
        }
        _ => read_vcf_to_matrix(&args, contig, 0, None, None)?,
    };

    ensure!(
        vcf.nsamples() == 1,
        "Input should only be a single sample id"
    );

    let ht1 = vcf.find_haplotype_for_sample(contig, 0..vcf.ncoords(), 0);
    let ht2 = vcf.find_haplotype_for_sample(contig, 0..vcf.ncoords(), 1);

    let sample_name = vcf.samples()[0].clone();
    let mut sh_output = args.output.clone();
    push_to_output(
        &args,
        &mut sh_output,
        &format!("{sample_name}_haplotype_1"),
        "csv",
    );
    let mut writer = open_csv_writer(sh_output)?;
    write_haplotype(&mut writer, ht1)?;

    let mut sh_output = args.output.clone();
    push_to_output(
        &args,
        &mut sh_output,
        &format!("{sample_name}_haplotype_2"),
        "csv",
    );
    let mut writer = open_csv_writer(sh_output)?;
    write_haplotype(&mut writer, ht2)?;

    Ok(())
}

#[doc(hidden)]
fn write_haplotype(
    writer: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
    haplotype: Vec<HapVariant>,
) -> Result<()> {
    for variant in haplotype {
        writer.write_record(vec![
            &variant.contig.to_string(),
            &variant.pos.to_string(),
            &variant.reference,
            &variant.alt,
            &variant.gt.to_string(),
        ])?;
    }
    Ok(())
}
