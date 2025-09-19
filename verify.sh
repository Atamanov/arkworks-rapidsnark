#!/bin/bash

echo "Verifying proof..."
echo "=================="
echo ""

# Check if rapidsnark-verifier is installed
if ! command -v rapidsnark-verifier &> /dev/null; then
    echo "Error: rapidsnark-verifier not found!"
    echo "Please install it with:"
    echo "  git clone https://github.com/iden3/rapidsnark.git"
    echo "  cd rapidsnark && npm install"
    echo "  git submodule init && git submodule update"
    echo "  npx task createFieldSources"
    echo "  npx task buildVerifier"
    echo "  sudo cp package_macos_arm64/bin/verifier /usr/local/bin/rapidsnark-verifier"
    exit 1
fi

# Check if proof exists
if [ ! -f "outputs/proof.json" ]; then
    echo "Error: No proof found!"
    echo "Please run: ./prove.sh"
    exit 1
fi

# Verify with rapidsnark
echo "Verifying with rapidsnark..."
rapidsnark-verifier outputs/verification_key.json outputs/public.json outputs/proof.json

if [ $? -eq 0 ]; then
    echo ""
    
    # Also verify with snarkjs for comparison
    echo "Cross-checking with snarkjs..."
    npx snarkjs groth16 verify outputs/verification_key.json outputs/public.json outputs/proof.json
    
    echo ""
    echo "✅ Proof verification successful!"
else
    echo ""
    echo "❌ Proof verification failed!"
    exit 1
fi