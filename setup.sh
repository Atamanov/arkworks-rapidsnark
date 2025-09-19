#!/bin/bash

# Setup script for arkworks-rapidsnark pipeline

echo "Setting up arkworks-rapidsnark pipeline..."
echo "=========================================="
echo ""

# 1. Generate R1CS and witness from arkworks circuit
echo "Step 1: Generating R1CS and witness files..."
cargo run
if [ $? -ne 0 ]; then
    echo "Error: Failed to generate R1CS/witness files"
    exit 1
fi

# 2. Verify R1CS structure
echo ""
echo "Step 2: Verifying R1CS structure..."
npx snarkjs r1cs info outputs/circuit.r1cs
if [ $? -ne 0 ]; then
    echo "Error: Failed to verify R1CS"
    exit 1
fi

# 3. Use existing Powers of Tau file (or generate if needed)
PTAU_FILE="pot8_final.ptau"
if [ ! -f "$PTAU_FILE" ]; then
    echo ""
    echo "Step 3: Generating Powers of Tau locally..."
    echo "This file supports circuits up to 2^8 constraints"
    npx snarkjs powersoftau new bn128 8 pot8_0000.ptau
    npx snarkjs powersoftau contribute pot8_0000.ptau pot8_0001.ptau --name="First" -e="random"
    npx snarkjs powersoftau prepare phase2 pot8_0001.ptau pot8_final.ptau
    if [ $? -ne 0 ]; then
        echo "Error: Failed to generate Powers of Tau"
        exit 1
    fi
    echo "Powers of Tau generated successfully"
else
    echo ""
    echo "Step 3: Powers of Tau file already exists, skipping generation..."
fi

# 4. Perform trusted setup
echo ""
echo "Step 4: Performing trusted setup..."
npx snarkjs groth16 setup outputs/circuit.r1cs $PTAU_FILE outputs/circuit_0000.zkey
if [ $? -ne 0 ]; then
    echo "Error: Failed to perform trusted setup"
    exit 1
fi

# 5. Contribute to ceremony (optional for production, but good for this demo)
echo ""
echo "Step 5: Contributing to ceremony..."
npx snarkjs zkey contribute outputs/circuit_0000.zkey outputs/circuit_final.zkey --name="Main" -e="random"
if [ $? -ne 0 ]; then
    echo "Error: Failed to contribute to ceremony"
    exit 1
fi

# 6. Export verification key
echo ""
echo "Step 6: Exporting verification key..."
npx snarkjs zkey export verificationkey outputs/circuit_final.zkey outputs/verification_key.json
if [ $? -ne 0 ]; then
    echo "Error: Failed to export verification key"
    exit 1
fi

echo ""
echo "✅ Setup complete! The pipeline is ready."
echo ""
echo "You can now:"
echo "  - Generate proofs with: ./prove.sh"
echo "  - Verify proofs with: ./verify.sh"