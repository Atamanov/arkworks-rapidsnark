use ark_bn254::Fr;
use ark_ff::{PrimeField, BigInteger};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem};
use r1cs_file::{FieldElement as R1csFE, Header, Constraint as R1csConstraint, Constraints, R1csFile, WireMap};
use wtns_file::{WtnsFile, FieldElement as WtnsFE};
use std::fs::File;
use anyhow::Result;

const FIELD_SIZE: usize = 32; // BN254 elements are 32 bytes

/// Export an arkworks circuit to Circom-compatible R1CS and WTNS files
pub fn export_to_circom_files<C: ConstraintSynthesizer<Fr>>(
    circuit: C,
    r1cs_path: &str,
    wtns_path: &str,
) -> Result<()> {
    // 1) Build constraint system and synthesize
    let cs = ConstraintSystem::<Fr>::new_ref();
    circuit.generate_constraints(cs.clone())?;
    cs.finalize();

    // 2) Get matrices and dimensions
    let matrices = cs.to_matrices().expect("Failed to get matrices");
    let num_constraints = matrices.num_constraints;
    let num_instance = matrices.num_instance_variables; // public vars
    let num_witness = matrices.num_witness_variables;   // private vars

    println!("Circuit stats:");
    println!("  Constraints: {}", num_constraints);
    println!("  Public inputs: {}", num_instance);
    println!("  Private inputs: {}", num_witness);

    // 3) Create R1CS header
    // Get BN254 prime field modulus as little-endian bytes (SnarkJS expects LE)
    let prime_bytes = <Fr as PrimeField>::MODULUS.to_bytes_le();
    let mut prime_le_32 = [0u8; FIELD_SIZE];
    prime_le_32[..prime_bytes.len()].copy_from_slice(&prime_bytes);
    let prime = R1csFE::<FIELD_SIZE>::from(prime_le_32);

    // Arkworks already counts the constant 1 in num_instance
    let n_wires = num_instance + num_witness;
    let header = Header::<FIELD_SIZE> {
        prime,
        n_wires: n_wires as u32,
        n_pub_out: 0,                      // arkworks doesn't distinguish outputs
        n_pub_in: num_instance as u32,     // all public vars are inputs
        n_prvt_in: num_witness as u32,     // all private vars
        n_labels: n_wires as u64,
        n_constraints: num_constraints as u32,
    };

    // 4) Convert arkworks matrices to Circom format
    // Helper to convert sparse row to Circom terms
    let to_terms = |row: &Vec<(Fr, usize)>| -> Vec<(R1csFE<FIELD_SIZE>, u32)> {
        row.iter()
            .map(|(coeff, col_idx)| {
                // Convert coefficient to little-endian bytes (SnarkJS expects LE)
                let coeff_bytes = coeff.into_bigint().to_bytes_le();
                let mut le32 = [0u8; FIELD_SIZE];
                le32[..coeff_bytes.len()].copy_from_slice(&coeff_bytes);
                (R1csFE::<FIELD_SIZE>::from(le32), *col_idx as u32)
            })
            .collect()
    };

    // Convert all constraints
    let mut constraints = Vec::with_capacity(num_constraints);
    for i in 0..num_constraints {
        let a_terms = to_terms(&matrices.a[i]);
        let b_terms = to_terms(&matrices.b[i]);
        let c_terms = to_terms(&matrices.c[i]);
        constraints.push(R1csConstraint::<FIELD_SIZE>(a_terms, b_terms, c_terms));
    }

    // 5) Write R1CS file
    let mut r1cs_file = File::create(r1cs_path)?;
    R1csFile::<FIELD_SIZE> {
        header,
        constraints: Constraints(constraints),
        map: WireMap::default(),
    }.write(&mut r1cs_file)?;
    println!("Written R1CS to: {}", r1cs_path);

    // 6) Build witness vector in Circom order
    // IMPORTANT: Arkworks includes the constant 1 as the first element in instance_assignment
    // So the order is already: [1, public_inputs...] in instance_values
    // and [private_inputs...] in witness_values
    let cs_borrow = cs.borrow().unwrap();
    let instance_values = cs_borrow.instance_assignment.clone();
    let witness_values = cs_borrow.witness_assignment.clone();

    let mut witness_vector = Vec::with_capacity(n_wires);
    
    // Instance values already include the constant 1 as first element
    // Convert from Montgomery form to standard form
    for val in instance_values {
        // val is already in standard form when retrieved from constraint system
        let val_bytes = val.into_bigint().to_bytes_le();
        let mut le32 = [0u8; FIELD_SIZE];
        le32[..val_bytes.len()].copy_from_slice(&val_bytes);
        witness_vector.push(WtnsFE::<FIELD_SIZE>::from(le32));
    }

    // Private inputs - also convert from Montgomery form
    for val in witness_values {
        let val_bytes = val.into_bigint().to_bytes_le();
        let mut le32 = [0u8; FIELD_SIZE];
        le32[..val_bytes.len()].copy_from_slice(&val_bytes);
        witness_vector.push(WtnsFE::<FIELD_SIZE>::from(le32));
    }

    // 7) Write WTNS file
    let mut wtns_file = File::create(wtns_path)?;
    // Need to create WTNS prime from same bytes (little-endian)
    let prime_bytes_wtns = <Fr as PrimeField>::MODULUS.to_bytes_le();
    let mut prime_le_32_wtns = [0u8; FIELD_SIZE];
    prime_le_32_wtns[..prime_bytes_wtns.len()].copy_from_slice(&prime_bytes_wtns);
    let prime_wtns = WtnsFE::<FIELD_SIZE>::from(prime_le_32_wtns);
    WtnsFile::<FIELD_SIZE>::from_vec(witness_vector, prime_wtns).write(&mut wtns_file)?;
    println!("Written witness to: {}", wtns_path);

    Ok(())
}