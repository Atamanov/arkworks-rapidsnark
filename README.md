# Arkworks to Rapidsnark Pipeline

This project demonstrates how to prove arkworks circuits with rapidsnark by serializing arkworks constraint systems to Circom-compatible R1CS and WTNS formats. The pipeline is fully functional and can generate and verify proofs using rapidsnark.

## Overview

The pipeline enables you to:
1. Write circuits in Rust using arkworks
2. Export them to Circom-compatible formats (R1CS and WTNS)
3. Perform trusted setup with snarkjs
4. Generate proofs with rapidsnark (much faster than snarkjs)

## Key Components

### Circuit (`src/circuit.rs`)
A simple demonstration circuit that proves knowledge of private inputs `a` and `b` such that:
- `a * b = c` (public output)
- `a + b = d` (public output)

### Serializer (`src/serializer.rs`)
Converts arkworks constraint systems to Circom formats:
- Exports R1CS binary format for constraints
- Exports WTNS binary format for witness
- Handles endianness conversion (Circom uses little-endian)
- Maintains proper wire ordering (constant 1, public inputs, private inputs)

## Prerequisites

### 1. Install Rapidsnark

Rapidsnark provides fast proof generation and verification. Install it system-wide:

```bash
# Clone and build rapidsnark
git clone https://github.com/iden3/rapidsnark.git
cd rapidsnark
npm install
git submodule init
git submodule update
npx task createFieldSources
npx task buildProver
npx task buildVerifier

# Install system-wide (macOS ARM64)
sudo cp package_macos_arm64/bin/prover /usr/local/bin/rapidsnark-prover
sudo cp package_macos_arm64/bin/verifier /usr/local/bin/rapidsnark-verifier

# For other platforms, replace package_macos_arm64 with:
# - package_linux_amd64 (Linux x86_64)
# - package_macos_amd64 (macOS Intel)
```

### 2. Install SnarkJS

```bash
npm install -g snarkjs
```

## Quick Start

Use the provided scripts for a streamlined workflow:

### 1. Run Complete Setup
```bash
./setup.sh
```

This script will:
- Generate R1CS and witness files from the arkworks circuit
- Generate or use existing Powers of Tau file
- Perform trusted setup
- Export verification key

### 2. Generate Proof with Rapidsnark
```bash
./prove.sh
```

Generates a proof using rapidsnark (much faster than snarkjs).

### 3. Verify Proof
```bash
./verify.sh
```

Verifies the proof using both rapidsnark and snarkjs for cross-validation.

## Manual Usage

If you prefer to run commands manually:

### 1. Generate R1CS and Witness Files
```bash
cargo run
```
This creates:
- `outputs/circuit.r1cs` - The constraint system
- `outputs/witness.wtns` - The witness values

### 2. Generate or Download Powers of Tau
```bash
# Option A: Generate locally (for small circuits)
npx snarkjs powersoftau new bn128 8 pot8_0000.ptau
npx snarkjs powersoftau contribute pot8_0000.ptau pot8_0001.ptau --name="First" -e="random"
npx snarkjs powersoftau prepare phase2 pot8_0001.ptau pot8_final.ptau

# Option B: Download from a trusted ceremony (when available)
# Various ceremonies provide ptau files for different circuit sizes
```

### 3. Perform Trusted Setup
```bash
# Create proving key
npx snarkjs groth16 setup outputs/circuit.r1cs pot8_final.ptau outputs/circuit_0000.zkey
npx snarkjs zkey contribute outputs/circuit_0000.zkey outputs/circuit_final.zkey --name="Main" -e="random"
```

### 4. Generate Proof with Rapidsnark
```bash
rapidsnark-prover outputs/circuit_final.zkey outputs/witness.wtns outputs/proof.json outputs/public.json
```

### 5. Verify Proof
```bash
# Export verification key
npx snarkjs zkey export verificationkey outputs/circuit_final.zkey outputs/verification_key.json

# Verify with rapidsnark
rapidsnark-verifier outputs/verification_key.json outputs/public.json outputs/proof.json

# Cross-verify with snarkjs
npx snarkjs groth16 verify outputs/verification_key.json outputs/public.json outputs/proof.json
```

## Technical Details

### Field Element Encoding
- Uses BN254 curve (same as Circom/SnarkJS)
- Field elements are 32 bytes, little-endian encoded
- Prime field modulus must match exactly

### Wire Ordering
The witness vector follows Circom convention:
1. Wire 0: Constant value 1
2. Wires 1..n: Public inputs
3. Wires n+1..m: Private inputs

### Matrix Format
Arkworks matrices (A, B, C) are converted from sparse row format to Circom's term list format, where each constraint is represented as a list of (coefficient, wire_index) pairs.

## Dependencies
- arkworks (ark-bn254, ark-ff, ark-relations, ark-r1cs-std)
- r1cs-file and wtns-file crates for Circom format serialization
- snarkjs for trusted setup
- rapidsnark for fast proving

## Performance

Rapidsnark provides significantly faster proof generation compared to snarkjs, especially for larger circuits. The serialization overhead is minimal compared to the proving time savings.

- **Proof Generation**: Rapidsnark is typically 5-10x faster than snarkjs
- **Proof Verification**: Both rapidsnark and snarkjs provide fast verification
- **Serialization**: The arkworks to Circom format conversion adds minimal overhead

## Troubleshooting

### "rapidsnark-prover not found"
Make sure you've installed rapidsnark system-wide as described in Prerequisites.

### "Powers of Tau file invalid"
Ensure you're using a compatible Powers of Tau file. The Hermez ceremony files are well-tested and recommended.

### Proof verification fails
Check that:
1. The witness file matches the R1CS constraints
2. The proving key was generated from the same R1CS
3. Public inputs are correctly ordered

---

Buy me a coffee (ERC20): 0x5B6580881E6F7888B4648402b29582Cd2C73BF7f