use ark_bn254::Fr;
use ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystemRef, SynthesisError,
};
use ark_r1cs_std::prelude::*;
use ark_r1cs_std::fields::fp::FpVar;

/// Simple demo circuit: proves knowledge of a and b such that:
/// a * b = c (public output)
/// a + b = d (public output)
#[derive(Clone)]
pub struct SimpleCircuit {
    /// Private input
    pub a: Option<Fr>,
    /// Private input
    pub b: Option<Fr>,
    /// Public output: a * b
    pub c: Option<Fr>,
    /// Public output: a + b
    pub d: Option<Fr>,
}

impl ConstraintSynthesizer<Fr> for SimpleCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        // Allocate private inputs
        let a_var = FpVar::new_witness(cs.clone(), || {
            self.a.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        let b_var = FpVar::new_witness(cs.clone(), || {
            self.b.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Allocate public outputs
        let c_var = FpVar::new_input(cs.clone(), || {
            self.c.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        let d_var = FpVar::new_input(cs.clone(), || {
            self.d.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Enforce constraints
        // a * b = c
        let ab = &a_var * &b_var;
        ab.enforce_equal(&c_var)?;
        
        // a + b = d
        let a_plus_b = &a_var + &b_var;
        a_plus_b.enforce_equal(&d_var)?;
        
        Ok(())
    }
}