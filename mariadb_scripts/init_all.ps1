# PowerShell script to initialize ImgCat database
# Equivalent to init_all.sh

# Create the schemas we'll need
Get-Content "init\init_db.sql"
Write-Output "" # Add empty line between sections

# Load the tables in dependency order
Get-Content "init\userdb.tables.sql"     # dep: n/a
Get-Content "init\posts.tables.sql"      # dep: userdb
Get-Content "init\comments.tables.sql"   # dep: posts
Get-Content "init\fpcache.tables.sql"    # dep: posts
Get-Content "init\actions.tables.sql"    # dep: favorites
Write-Output "" # Add empty line between sections

# Order doesn't matter. SPs are checked at runtime, not creation
# Get all .sql files in init directory, excluding the ones we want to skip
$sqlFiles = Get-ChildItem "init\*.sql" | Where-Object {
    $_.Name -ne "init_db.sql" -and 
    $_.Name -ne "init_perms.sql" -and 
    $_.Name -notlike "*.tables.sql"
}

# Output each SQL file with empty line separator
foreach ($file in $sqlFiles) {
    Get-Content $file.FullName
    Write-Output "" # Add empty line between files
}
Write-Output "" # Add empty line between sections

# Once all the SPs are created, run perms
# TODO: We can/should put the GRANT permissions in each script
# We just need an envvar, in case people change the user
Get-Content "init\init_perms.sql"
Write-Output "" # Add empty line between sections
