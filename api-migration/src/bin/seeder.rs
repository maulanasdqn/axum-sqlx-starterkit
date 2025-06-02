use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

    println!("🌱 Starting database seeding...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("Failed to connect to database")?;

    println!("✅ Connected to database");

    seed_roles(&pool).await?;
    seed_permissions(&pool).await?;
    seed_role_permissions(&pool).await?;
    seed_users(&pool).await?;

    println!("🎉 All seeders completed successfully!");
    println!("\n📊 Seeded data summary:");
    print_summary(&pool).await?;

    Ok(())
}

async fn seed_roles(pool: &Pool<Postgres>) -> Result<()> {
    println!("📝 Seeding roles...");
    let sql = include_str!("../seeder/seed_role.sql");
    sqlx::query(sql)
        .execute(pool)
        .await
        .context("Failed to seed roles")?;
    println!("✅ Roles seeded");
    Ok(())
}

async fn seed_permissions(pool: &Pool<Postgres>) -> Result<()> {
    println!("🔐 Seeding permissions...");
    let sql = include_str!("../seeder/seed_permission.sql");
    sqlx::query(sql)
        .execute(pool)
        .await
        .context("Failed to seed permissions")?;
    println!("✅ Permissions seeded");
    Ok(())
}

async fn seed_role_permissions(pool: &Pool<Postgres>) -> Result<()> {
    println!("🔗 Seeding role-permission mappings...");
    let sql = include_str!("../seeder/seed_role_permissions.sql");
    sqlx::query(sql)
        .execute(pool)
        .await
        .context("Failed to seed role permissions")?;
    println!("✅ Role-permission mappings seeded");
    Ok(())
}

async fn seed_users(pool: &Pool<Postgres>) -> Result<()> {
    println!("👥 Seeding users...");
    let sql = include_str!("../seeder/seed_user.sql");
    sqlx::query(sql)
        .execute(pool)
        .await
        .context("Failed to seed users")?;
    println!("✅ Users seeded");
    Ok(())
}

async fn print_summary(pool: &Pool<Postgres>) -> Result<()> {
    let role_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM app_roles")
        .fetch_one(pool)
        .await?;

    let permission_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM app_permissions")
        .fetch_one(pool)
        .await?;

    let user_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM app_users")
        .fetch_one(pool)
        .await?;

    let mapping_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM app_role_permissions")
        .fetch_one(pool)
        .await?;

    println!("  - {} roles created", role_count.0);
    println!("  - {} permissions created", permission_count.0);
    println!("  - {} users created", user_count.0);
    println!("  - {} role-permission mappings created", mapping_count.0);

    println!("\n🔑 Sample login credentials:");
    println!("  Super Admin: superadmin@example.com / password123");
    println!("  Admin:       admin@example.com / password123");
    println!("  Manager:     manager@example.com / password123");
    println!("  User:        user@example.com / password123");
    println!("  Guest:       guest@example.com / password123");

    Ok(())
}
