# PowerShell script to generate self-signed certificates for WebTransport development
# Run this script from the project root directory

$ErrorActionPreference = "Stop"

$CertDir = "certs"
$Days = 30
$Domain = "localhost"

Write-Host "Creating certificate directory..."
New-Item -ItemType Directory -Force -Path $CertDir | Out-Null

Write-Host "Generating self-signed certificate for $Domain..."
openssl req -x509 `
    -newkey ec `
    -pkeyopt ec_paramgen_curve:prime256v1 `
    -keyout "$CertDir/key.pem" `
    -out "$CertDir/cert.pem" `
    -days $Days `
    -nodes `
    -subj "/CN=$Domain"

Write-Host ""
Write-Host "Certificate generated successfully!" -ForegroundColor Green
Write-Host "  - Certificate: $CertDir/cert.pem"
Write-Host "  - Private Key: $CertDir/key.pem"
Write-Host "  - Valid for: $Days days"
Write-Host ""

Write-Host "Certificate fingerprint (SHA-256):"
$certDer = openssl x509 -in "$CertDir/cert.pem" -outform der
$certDer | openssl dgst -sha256

Write-Host ""
Write-Host "To run the server:"
Write-Host "  cargo run -p khanhtimn_dev_server -- --cert $CertDir/cert.pem --key $CertDir/key.pem"
