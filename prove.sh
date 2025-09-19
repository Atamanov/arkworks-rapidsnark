#!/bin/bash

echo "Generating proof with Rapidsnark..."
echo "===================================="
echo ""

# Check if rapidsnark-prover is installed
if ! command -v rapidsnark-prover &> /dev/null; then
    echo "Error: rapidsnark-prover not found!"
    echo "Please install it with:"
    echo "  git clone https://github.com/iden3/rapidsnark.git"
    echo "  cd rapidsnark && npm install"
    echo "  git submodule init && git submodule update"
    echo "  npx task createFieldSources"
    echo "  npx task buildProver"
    echo "  sudo cp package_macos_arm64/bin/prover /usr/local/bin/rapidsnark-prover"
    exit 1
fi

# Check if setup has been run
if [ ! -f "outputs/circuit_final.zkey" ]; then
    echo "Error: Setup has not been run yet!"
    echo "Please run: ./setup.sh"
    exit 1
fi

# Generate proof with rapidsnark
echo "Generating proof..."
rapidsnark-prover outputs/circuit_final.zkey outputs/witness.wtns outputs/proof_raw.json outputs/public_raw.json

if [ $? -eq 0 ]; then
    # Clean JSON files by removing null bytes that rapidsnark adds
    echo "Cleaning JSON output..."
    tr -d '\000' < outputs/proof_raw.json > outputs/proof.json
    tr -d '\000' < outputs/public_raw.json > outputs/public.json
    
    echo ""
    echo "✅ Proof generated successfully!"
    echo "  - Proof: outputs/proof.json"
    echo "  - Public signals: outputs/public.json"
    echo ""
    echo "To verify the proof, run: ./verify.sh"
else
    echo ""
    echo "❌ Failed to generate proof"
    exit 1
fi