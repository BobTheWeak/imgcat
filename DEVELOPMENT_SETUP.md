# ImgCat Development Environment Setup Guide

This guide provides step-by-step instructions for setting up a complete development environment for ImgCat, an open-source photo-sharing community platform.

## Prerequisites

### Required Software
- **Node.js** (v18 or later) - [Download from nodejs.org](https://nodejs.org/)
- **MariaDB Server** (v10.6 or later) - [Download from mariadb.org](https://mariadb.org/download/)
- **Git** - [Download from git-scm.com](https://git-scm.com/)
- **S3-compatible storage** - One of:
  - **MinIO** (local development) - [Download from min.io](https://min.io/download)
  - **NAS with S3 support** (Synology, QNAP, etc.)
  - **Cloudflare R2** (production)

### System Requirements
- Minimum 4GB RAM
- At least 2GB free disk space
- Modern web browser for testing

## Step 1: Clone and Initial Setup

1. Clone the repository:
```bash
# Bash/Linux/macOS
git clone <repository-url>
cd imgcat
```
```powershell
# PowerShell/Windows
git clone <repository-url>
Set-Location imgcat
```

2. Install Node.js dependencies:
```bash
# Bash/Linux/macOS/Windows
npm install
```

## Step 2: Database Setup

### 2.1: MariaDB Installation and Configuration

1. Install MariaDB Server with default settings
2. During installation, set a strong root password
3. Ensure MariaDB service is running

### 2.2: Database Initialization

The database setup requires two main steps: creating users and initializing the schema.

#### Create Database Users

1. Navigate to the MariaDB scripts directory:
```bash
# Bash/Linux/macOS
cd mariadb_scripts
```
```powershell
# PowerShell/Windows
Set-Location mariadb_scripts
```

2. Edit `init_users.sql` to add secure passwords:
```sql
CREATE USER imgcat_jwt
IDENTIFIED BY 'your_strong_jwt_password_here'
WITH MAX_STATEMENT_TIME 1;

CREATE USER imgcat_posts
IDENTIFIED BY 'your_strong_posts_password_here'
WITH MAX_STATEMENT_TIME 5;

CREATE USER imgcat_mods
IDENTIFIED BY 'your_strong_mods_password_here'
WITH MAX_STATEMENT_TIME 5;

CREATE USER imgcat_system
IDENTIFIED BY 'your_strong_system_password_here'
WITH MAX_STATEMENT_TIME 5;
```

3. Run the user creation script:
```bash
# Bash/Linux/macOS
mariadb -h localhost -u root -p < init_users.sql
```
```powershell
# PowerShell/Windows
Get-Content init_users.sql | mariadb -h localhost -u root -p
```
*Enter your MariaDB root password when prompted*

**Note for Windows PowerShell users:** If you get an execution policy error when running the `.ps1` script, you may need to temporarily allow script execution:
```powershell
# Allow scripts for current session only
Set-ExecutionPolicy -ExecutionPolicy Bypass -Scope Process
# Then run the database initialization
.\init_all.ps1 | mariadb -h localhost -u root -p
```

#### Initialize Database Schema

4. Run the database initialization:
```bash
# Bash/Linux/macOS
./init_all.sh | mariadb -h localhost -u root -p
```
```powershell
# PowerShell/Windows
.\init_all.ps1 | mariadb -h localhost -u root -p

# OR use bash if available (Git Bash/WSL)
bash init_all.sh | mariadb -h localhost -u root -p

# OR fallback method
Get-Content init_all.sh | mariadb -h localhost -u root -p
```
*Enter your MariaDB root password when prompted*

## Step 3: Environment Configuration

1. Copy the example environment file:
```bash
# Bash/Linux/macOS
cd ..
cp .env .env.local
```
```powershell
# PowerShell/Windows
Set-Location ..
Copy-Item .env .env.local
```

2. Edit `.env` with your development settings:

```env
# App Database Connection
DB_APP_HOST=localhost
DB_APP_PORT=3306
DB_APP_USER=imgcat_posts
DB_APP_PASS=your_strong_posts_password_here

# User Database Connection  
DB_USERDB_HOST=localhost
DB_USERDB_PORT=3306
DB_USERDB_USER=imgcat_jwt
DB_USERDB_PASS=your_strong_jwt_password_here

# Image serving configuration for development
IMAGE_BASE_URL=http://localhost:5173/uploads
BASE_URL=http://localhost:5173

# Storage mode - 'local' for development, 'r2' for production
STORAGE_MODE=local
LOCAL_STORAGE_PATH=./uploads

# Cloudflare R2 (for production - leave commented for local development)
# When using R2, set STORAGE_MODE=r2 and uncomment the lines below:
# CFR2_ACCOUNT_ID=your_account_id
# CFR2_ACCESS_KEY_ID=your_access_key
# CFR2_SECRET_ACCESS_KEY=your_secret_key
# CFR2_BUCKET=your_bucket_name
```

3. Create the uploads directory:
```bash
# Bash/Linux/macOS
mkdir uploads
```
```powershell
# PowerShell/Windows
New-Item -ItemType Directory -Path uploads
```

## Step 4: S3-Compatible Storage Setup

Choose one of the following storage options:

### Option A: MinIO (Recommended for Development)

1. **Install MinIO**:
   - Download from [min.io/download](https://min.io/download)
   - Or use Docker: `docker run -p 9000:9000 -p 9001:9001 minio/minio server /data --console-address ":9001"`

2. **Start MinIO**:
   ```bash
   # Default credentials: minioadmin/minioadmin
   minio server /path/to/data --console-address ":9001"
   ```

3. **Create bucket**:
   - Open MinIO Console at `http://localhost:9001`
   - Login with `minioadmin` / `minioadmin`
   - Create bucket named `imgcat-dev`
   - Set bucket to public read access

### Option B: NAS Storage (Synology/QNAP)

1. **Enable S3 service** on your NAS
2. **Create access credentials** in your NAS admin panel
3. **Create bucket** named `imgcat-dev`
4. **Update .env** with your NAS endpoint and credentials
   ```env
   S3_ENDPOINT=http://your-nas-ip:port
   S3_IGNORE_SSL=true  # For self-signed certificates
   S3_FORCE_HTTP=true  # recommended for easy development
   CFR2_ACCESS_KEY_ID=your_nas_access_key
   CFR2_SECRET_ACCESS_KEY=your_nas_secret_key
   ```
5. **Set bucket to Public** 
6. **Set Image url from public URL**
   ```env
   IMAGE_BASE_URL=http://your-nas-ip:port/v1/AUTH_Imgcat-Dev/imgcat-dev
   ```

### Option C: Cloudflare R2 (Production)

1. **Create R2 bucket** in Cloudflare dashboard
2. **Generate API tokens** with R2 permissions
3. **Update .env** with R2 credentials

## Step 5: JWT Certificate Generation

ImgCat uses JWT (JSON Web Tokens) for authentication, which requires cryptographic certificates.

### Generate JWT Certificates

```bash
# Bash/Linux/macOS
./generate-jwt-certs.sh
```
```powershell
# PowerShell/Windows
.\generate-jwt-certs.ps1
```

**Note:** You need OpenSSL installed for certificate generation:
- **Windows**: Download from [Win32 OpenSSL](https://slproweb.com/products/Win32OpenSSL.html) or use Git Bash
- **Linux**: Usually pre-installed (`sudo apt install openssl` or `sudo yum install openssl`)
- **macOS**: Pre-installed or via Homebrew (`brew install openssl`)

### Verify Certificate Generation

After running the script, you should see:
- `certs/jwt-private.pem` - Private key for JWT signing
- `certs/jwt-public.pem` - Public key for JWT verification
- Updated `.env` with JWT configuration

### Storage Configuration

ImgCat uses S3-compatible storage for uploaded images. Two options are supported:

**Local MinIO/NAS (Development):**
- S3-compatible server running on your local network
- Examples: MinIO, Synology NAS, QNAP NAS
- Configuration: Set `S3_ENDPOINT` to your local server
- Default: `http://localhost:9000` (standard MinIO port)

**Cloudflare R2 (Production):**
- Cloudflare's S3-compatible storage service
- Requires Cloudflare account and R2 credentials
- Scalable for production use
- Configuration: Use your actual R2 account ID

### SSL Certificate Handling

For local development with self-signed certificates or when SSL verification needs to be bypassed:

**Option 1: Ignore SSL Certificates**
```env
# Ignore SSL certificate errors (development only)
S3_IGNORE_SSL=true
```

**Option 2: Force HTTP (Alternative)**
```env
# Force HTTP instead of HTTPS (development only) 
S3_FORCE_HTTP=true
# This will convert https://your-nas:port to http://your-nas:port
```

**Option 3: Use HTTP Endpoint Directly**
```env
# Set endpoint to HTTP instead of HTTPS
S3_ENDPOINT=http://your-nas-ip:port
```

**⚠️ Security Note**: Only use these SSL bypass options for local development. Never use these settings in production as they make connections vulnerable to security issues.

## Step 6: Verification and Testing

### 4.1: Database Connection Test

1. Start the development server:
```bash
npm run dev
```

2. Check the console output for database connection messages:
   - "App pool started" - Main database connected
   - "UserDB pool started" - User database connected

3. If you see connection errors, verify:
   - MariaDB service is running
   - Database credentials in `.env` are correct
   - Users were created successfully

### 4.2: Application Test

1. Open your browser to `http://localhost:5173` *or link provided by NPM in the console
2. The application should load without errors
3. Check browser developer console for any JavaScript errors

## Step 7: Development Workflow

### Starting Development
```bash
# Bash/Linux/macOS/Windows
npm run dev
```

### Building for Production
```bash
# Bash/Linux/macOS/Windows
npm run build
```

### Running Production Preview
```bash
# Bash/Linux/macOS/Windows
npm run preview
```

### Code Quality Checks
```bash
# Bash/Linux/macOS/Windows
npm run check
```

## Troubleshooting

### Common Issues

#### Database Connection Errors
- **Error**: "Could not establish connection"
  - **Solution**: Verify MariaDB is running and credentials are correct
  - Check MariaDB error logs for details

#### Port Already in Use
- **Error**: "Port 5173 already in use"
  - **Solution**: Kill existing process or use different port:
    ```bash
    # Bash/Linux/macOS/Windows
    npm run dev -- --port 3000
    ```


#### Missing Dependencies
- **Error**: Module not found errors
  - **Solution**: Reinstall dependencies:
    ```bash
    # Bash/Linux/macOS
    rm -rf node_modules package-lock.json
    npm install
    ```
    ```powershell
    # PowerShell/Windows
    Remove-Item -Recurse -Force node_modules, package-lock.json
    npm install
    ```

#### JWT Certificate Errors
- **Error**: "The 'path' argument must be of type string... Received undefined"
  - **Solution**: Generate JWT certificates:
    ```bash
    # Run the certificate generation script
    ./generate-jwt-certs.sh   # Linux/macOS
    .\generate-jwt-certs.ps1  # Windows
    ```
  - **Alternative**: Manually create certificates:
    ```bash
    mkdir certs
    openssl genpkey -algorithm Ed25519 -out certs/jwt-private.pem
    openssl pkey -in certs/jwt-private.pem -pubout -out certs/jwt-public.pem
    ```

- **Error**: "Cannot find module 'openssl'"
  - **Solution**: Install OpenSSL or use Git Bash on Windows

#### Image Upload Errors
- **Error**: "No value provided for input HTTP label: Bucket"
  - **Cause**: S3-compatible storage not configured or not running
  - **Solution**: 
    1. Ensure MinIO/NAS is running and accessible
    2. Verify bucket `imgcat-dev` exists
    3. Check credentials in `.env` file
    4. Test connection: `curl http://localhost:9000` (should respond)

- **Error**: Images not loading after upload
  - **Solution**: 
    1. Verify bucket has public read access
    2. Check `IMAGE_BASE_URL` in `.env` matches your storage endpoint
    3. Confirm files are actually uploaded to the bucket
    4. Test direct access: `http://localhost:9000/imgcat-dev/filename`

#### SSL Certificate Errors
- **Error**: "unable to verify the first certificate" or "self signed certificate" or "SSL routines:ssl3_read_bytes:sslv3 alert handshake failure"
  - **Cause**: NAS/MinIO using self-signed SSL certificates or SSL version mismatch
  - **Solution Options** (try in order):
    
    **1. Force HTTP instead of HTTPS:**
    ```env
    S3_FORCE_HTTP=true
    ```
    
    **2. Ignore SSL certificates:**
    ```env
    S3_IGNORE_SSL=true
    S3_FORCE_HTTP=false
    ```
    
    **3. Use HTTP endpoint directly:**
    ```env
    S3_ENDPOINT=http://localhost:9000  # Replace with your NAS IP
    IMAGE_BASE_URL=http://localhost:9000/imgcat-dev
    ```

### Database Schema Issues

If you need to reset the database:

1. Drop all ImgCat databases:
```sql
DROP DATABASE IF EXISTS Posts;
DROP DATABASE IF EXISTS UserDB;
DROP DATABASE IF EXISTS FPCache;
DROP DATABASE IF EXISTS Actions;
DROP DATABASE IF EXISTS Comments;
```

2. Drop all ImgCat users:
```sql
DROP USER IF EXISTS imgcat_jwt;
DROP USER IF EXISTS imgcat_posts;
DROP USER IF EXISTS imgcat_mods;
DROP USER IF EXISTS imgcat_system;
```

3. Re-run the initialization process from Step 2.2

## Development Notes

### File Structure
- `/src/routes/` - SvelteKit pages and API endpoints
- `/src/lib/` - Reusable components and utilities
- `/src/lib/server/` - Server-side utilities and database functions
- `/static/` - Static assets
- `/uploads/` - User-uploaded images (development only)

### Environment Separation
This development setup is completely isolated from production instances:
- Uses local database with separate credentials
- Serves images from local filesystem
- Runs on localhost with different port
- No external dependencies or shared resources

### Making Changes
- The development server supports hot reloading
- Changes to `.env` require restarting the dev server
- Database schema changes require re-running initialization scripts

## Next Steps

After completing this setup:
1. Explore the codebase structure
2. Run the application and test basic functionality
3. Review the existing API endpoints in `/src/routes/`
4. Check the database schema by examining files in `/mariadb_scripts/init/`

For production deployment, additional configuration will be needed for:
- Production database credentials
- Cloudflare R2 image storage
- Domain and SSL configuration
- Performance optimization settings
