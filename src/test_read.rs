use r1cs_file::R1csFile;
use std::fs::File;

pub fn test_read_r1cs(path: &str) -> anyhow::Result<()> {
    println!("Testing R1CS file read: {}", path);
    let mut file = File::open(path)?;
    let r1cs: R1csFile<32> = R1csFile::read(&mut file)?;
    
    println!("Successfully read R1CS file!");
    println!("  Prime field size: 32 bytes");
    println!("  Constraints: {}", r1cs.header.n_constraints);
    println!("  Public inputs: {}", r1cs.header.n_pub_in);
    println!("  Private inputs: {}", r1cs.header.n_prvt_in);
    println!("  Total wires: {}", r1cs.header.n_wires);
    
    Ok(())
}