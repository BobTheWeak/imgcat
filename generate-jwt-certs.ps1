# Generate JWT certificates for ImgCat development
# This script creates Ed25519 key pair for JWT signing

Write-Host "Generating JWT certificates for development..." -ForegroundColor Green

# Create certs directory if it doesn't exist
$certsDir = "certs"
if (!(Test-Path $certsDir)) {
    New-Item -ItemType Directory -Path $certsDir | Out-Null
    Write-Host "Created certs directory" -ForegroundColor Yellow
}

# Generate Ed25519 private key
Write-Host "Generating Ed25519 private key..." -ForegroundColor Yellow
$privateKeyPath = Join-Path $certsDir "jwt-private.pem"
$publicKeyPath = Join-Path $certsDir "jwt-public.pem"

try {
    # Generate private key
    & openssl genpkey -algorithm Ed25519 -out $privateKeyPath
    
    # Extract public key
    & openssl pkey -in $privateKeyPath -pubout -out $publicKeyPath
    
    Write-Host "✅ JWT certificates generated successfully!" -ForegroundColor Green
    Write-Host "Private key: $privateKeyPath" -ForegroundColor Cyan
    Write-Host "Public key: $publicKeyPath" -ForegroundColor Cyan
    
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Yellow
    Write-Host "1. Add the following lines to your .env file:"
    Write-Host ""
    Write-Host "# JWT Configuration" -ForegroundColor Gray
    Write-Host "JWT_CERT_PVT=$privateKeyPath" -ForegroundColor Gray
    Write-Host "JWT_CERT_PUB=$publicKeyPath" -ForegroundColor Gray
    Write-Host "JWT_ISS=imgcat-dev" -ForegroundColor Gray
    Write-Host "JWT_AUD=imgcat-dev" -ForegroundColor Gray
    Write-Host ""
    Write-Host "2. Restart your development server" -ForegroundColor Yellow
    
} catch {
    Write-Host "❌ Error generating certificates. Make sure OpenSSL is installed." -ForegroundColor Red
    Write-Host "You can install OpenSSL from: https://slproweb.com/products/Win32OpenSSL.html" -ForegroundColor Yellow
    Write-Host "Or use Git Bash which includes OpenSSL" -ForegroundColor Yellow
    
    Write-Host ""
    Write-Host "Alternative: Use Git Bash to run:" -ForegroundColor Yellow
    Write-Host "mkdir -p certs" -ForegroundColor Gray
    Write-Host "openssl genpkey -algorithm Ed25519 -out certs/jwt-private.pem" -ForegroundColor Gray
    Write-Host "openssl pkey -in certs/jwt-private.pem -pubout -out certs/jwt-public.pem" -ForegroundColor Gray
}
