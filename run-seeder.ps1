# Database Seeder Script for Axum SQLx Starter Kit (PowerShell)
# This script runs the Rust seeder binary to populate the database

Write-Host "🌱 Axum SQLx Starter Kit - Database Seeder" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Green

# Check if .env file exists
if (-not (Test-Path ".env")) {
    Write-Host "❌ Error: .env file not found!" -ForegroundColor Red
    Write-Host "Please create a .env file with your DATABASE_URL" -ForegroundColor Yellow
    Write-Host "Example: DATABASE_URL=postgresql://username:password@localhost/database_name" -ForegroundColor Yellow
    exit 1
}

# Load environment variables from .env file
Get-Content ".env" | ForEach-Object {
    if ($_ -match "^([^#][^=]+)=(.*)$") {
        [Environment]::SetEnvironmentVariable($matches[1], $matches[2], "Process")
    }
}

# Check if DATABASE_URL is set
$DATABASE_URL = [Environment]::GetEnvironmentVariable("DATABASE_URL", "Process")
if (-not $DATABASE_URL) {
    Write-Host "❌ Error: DATABASE_URL not set in .env file!" -ForegroundColor Red
    Write-Host "Please add DATABASE_URL to your .env file" -ForegroundColor Yellow
    exit 1
}

Write-Host "📋 Using database: $DATABASE_URL" -ForegroundColor Cyan
Write-Host ""

# Run migrations first
Write-Host "🔄 Running database migrations..." -ForegroundColor Yellow
$migrationResult = & sqlx migrate run --source api-migration/src

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Migration failed! Please check your database connection." -ForegroundColor Red
    exit 1
}

Write-Host "✅ Migrations completed" -ForegroundColor Green
Write-Host ""

# Run the seeder
Write-Host "🌱 Running database seeders..." -ForegroundColor Yellow
$seederResult = & cargo run -p api-migration --bin seeder

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "🎉 Database seeding completed successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "🚀 You can now start your API server with:" -ForegroundColor Cyan
    Write-Host "   cargo run --bin api" -ForegroundColor White
} else {
    Write-Host "❌ Seeding failed! Please check the error messages above." -ForegroundColor Red
    exit 1
}
