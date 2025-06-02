#!/bin/bash

# Database Seeder Script for Axum SQLx Starter Kit
# This script runs the Rust seeder binary to populate the database

echo "🌱 Axum SQLx Starter Kit - Database Seeder"
echo "=========================================="

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo "❌ Error: .env file not found!"
    echo "Please create a .env file with your DATABASE_URL"
    echo "Example: DATABASE_URL=postgresql://username:password@localhost/database_name"
    exit 1
fi

# Load environment variables
source .env

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "❌ Error: DATABASE_URL not set in .env file!"
    echo "Please add DATABASE_URL to your .env file"
    exit 1
fi

echo "📋 Using database: $DATABASE_URL"
echo ""

# Run migrations first
echo "🔄 Running database migrations..."
sqlx migrate run --source api-migration/src

if [ $? -ne 0 ]; then
    echo "❌ Migration failed! Please check your database connection."
    exit 1
fi

echo "✅ Migrations completed"
echo ""

# Run the seeder
echo "🌱 Running database seeders..."
cargo run -p api-migration --bin seeder

if [ $? -eq 0 ]; then
    echo ""
    echo "🎉 Database seeding completed successfully!"
    echo ""
    echo "🚀 You can now start your API server with:"
    echo "   cargo run --bin api"
else
    echo "❌ Seeding failed! Please check the error messages above."
    exit 1
fi
