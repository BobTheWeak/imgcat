#!/bin/bash
# Generate JWT certificates for ImgCat development
# This script creates Ed25519 key pair for JWT signing

echo "🔑 Generating JWT certificates for development..."

# Create certs directory if it doesn't exist
mkdir -p certs

# Generate Ed25519 private key
echo "Generating Ed25519 private key..."
openssl genpkey -algorithm Ed25519 -out certs/jwt-private.pem

# Extract public key
echo "Extracting public key..."
openssl pkey -in certs/jwt-private.pem -pubout -out certs/jwt-public.pem

echo "✅ JWT certificates generated successfully!"
echo "Private key: certs/jwt-private.pem"
echo "Public key: certs/jwt-public.pem"
echo ""
echo "Next steps:"
echo "1. Add the following lines to your .env file:"
echo ""
echo "# JWT Configuration"
echo "JWT_CERT_PVT=certs/jwt-private.pem"
echo "JWT_CERT_PUB=certs/jwt-public.pem"  
echo "JWT_ISS=imgcat-dev"
echo "JWT_AUD=imgcat-dev"
echo ""
echo "2. Restart your development server"
