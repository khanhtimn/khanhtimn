#!/bin/bash
# Generate self-signed certificates for WebTransport development
# Run this script from the project root directory

set -e

CERT_DIR="certs"
DAYS=30
DOMAIN="localhost"

echo "Creating certificate directory..."
mkdir -p "$CERT_DIR"

echo "Generating self-signed certificate for $DOMAIN..."
openssl req -x509 \
    -newkey ec \
    -pkeyopt ec_paramgen_curve:prime256v1 \
    -keyout "$CERT_DIR/key.pem" \
    -out "$CERT_DIR/cert.pem" \
    -days "$DAYS" \
    -nodes \
    -subj "/CN=$DOMAIN"

echo ""
echo "Certificate generated successfully!"
echo "  - Certificate: $CERT_DIR/cert.pem"
echo "  - Private Key: $CERT_DIR/key.pem"
echo "  - Valid for: $DAYS days"
echo ""

echo "Certificate fingerprint (SHA-256):"
openssl x509 -in "$CERT_DIR/cert.pem" -outform der | openssl dgst -sha256

echo ""
echo "To run the server:"
echo "  cargo run -p game_server -- --cert $CERT_DIR/cert.pem --key $CERT_DIR/key.pem"
