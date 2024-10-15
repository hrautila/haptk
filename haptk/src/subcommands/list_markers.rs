use std::{collections::BTreeSet, path::PathBuf};

use color_eyre::{
    eyre::{eyre, WrapErr},
    Result,
};
use serde::{Deserialize, Serialize};

use crate::{structs::Coord, subcommands::bhst::Metadata};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HstCoords {
    metadata: Metadata,
}

pub fn read_hst_coords(path: PathBuf) -> Result<BTreeSet<Coord>> {
    let file = std::fs::File::open(path.clone()).wrap_err(eyre!("Error opening {path:?}"))?;
    let reader = bgzip::BGZFReader::new(file)?;
    let hst: HstCoords = serde_json::from_reader(reader)?;

    Ok(hst.metadata.coords)
}

#[doc(hidden)]
pub fn run(path: PathBuf) -> Result<()> {
    let coords = read_hst_coords(path)?;
    println!("contig,pos,ref,alt");
    for coord in coords {
        println!(
            "{},{},{},{}",
            coord.contig, coord.pos, coord.reference, coord.alt
        );
    }
    Ok(())
}
