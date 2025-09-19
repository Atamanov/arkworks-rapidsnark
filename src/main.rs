mod circuit;
mod serializer;
mod test_read;

use ark_bn254::Fr;
use circuit::SimpleCircuit;
use serializer::export_to_circom_files;
use std::fs;

fn main() -> anyhow::Result<()> {
    println!("Arkworks to Rapidsnark Demo");
    println!("============================\n");

    // Create output directory
    fs::create_dir_all("outputs")?;

    // Example values: a=3, b=5
    // Expected: c = a*b = 15, d = a+b = 8
    let a = Fr::from(3u32);
    let b = Fr::from(5u32);
    let c = a * b; // 15
    let d = a + b; // 8

    println!("Creating circuit with:");
    println!("  a (private) = 3");
    println!("  b (private) = 5");
    println!("  c (public) = a * b = 15");
    println!("  d (public) = a + b = 8\n");

    // Create circuit instance
    let circuit = SimpleCircuit {
        a: Some(a),
        b: Some(b),
        c: Some(c),
        d: Some(d),
    };

    // Export to Circom formats
    export_to_circom_files(
        circuit,
        "outputs/circuit.r1cs",
        "outputs/witness.wtns",
    )?;

    println!("\nExport complete!");
    
    // Test reading the file back
    println!("\nTesting R1CS file readability...");
    test_read::test_read_r1cs("outputs/circuit.r1cs")?;
    println!("Next steps:");
    println!("1. Run setup: ./setup.sh");
    println!("2. Generate proof: ./prove.sh");
    println!("3. Verify proof: ./verify.sh");
    println!("");
    println!("For manual steps, see README.md");

    Ok(())
}
