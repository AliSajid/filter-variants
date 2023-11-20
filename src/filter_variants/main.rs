// A CLI Struct for Clap with values for chromosome, mimum base and maximum base


use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("trevor_pileup.tsv")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {

        // see if line is a string. If it is, proceed
        if let Ok(line) = line {
            // split line into columns
            let columns: Vec<&str> = line.split_whitespace().collect();

            // get the chromosome
            let chromosome = columns[0];

            // get the base
            let position = columns[1];

            // get the coverage
            let reference = columns[2];

            // get the variant count
            let consensus_base = columns[3];

            // get the variant frequency
            let consensus_quality = columns[4];

            // get the reference base
            let snp_quality = columns[5];

            // get the variant base
            let maximum_mapping_quality = columns[6];

            // get the variant quality
            let coverage = columns[7];

            // get the mapping quality
            let bases_within_reads = columns[8];

            // get the base quality
            let quality_values = columns[9];

            if chromosome == "11" {
                break;
            }
    
            if chromosome == "10" {
                println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", chromosome, position, reference, consensus_base, consensus_quality, snp_quality, maximum_mapping_quality, coverage, bases_within_reads, quality_values)
            }
        }

    
    }
    Ok(())
}