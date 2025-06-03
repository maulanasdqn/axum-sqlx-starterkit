use api_util::paginator;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct User {
    pub id: i32,
    pub email: String,
    pub fullname: String,
    pub is_deleted: bool,
}

fn main() {
    println!("=== Paginator Usage Examples ===\n");

    println!("1. Basic paginator syntax (exactly as requested):");
    let paginator_builder = paginator("users")
        .search(&["email", "fullname"])
        .filter_by("email")
        .filter("user-input-email")
        .filter_by("fullname")
        .filter("user-input-fullname");

    println!("✓ Paginator created with search and filters");
    println!("  Table: users");
    println!("  Search columns: email, fullname");
    println!("  Filters: email='user-input-email', fullname='user-input-fullname'");
    println!();

    println!("2. Different pagination configurations:");

    let config1 = paginator("products")
        .search(&["name", "description"])
        .filter_by("category")
        .filter("electronics")
        .page(1)
        .per_page(20);
    println!("✓ Products paginator: page 1, 20 items per page");

    let config2 = paginator("orders")
        .search(&["order_id", "customer_name"])
        .filter_by("status")
        .filter("pending")
        .filter_by("created_date")
        .filter("2024-01-01")
        .page(2)
        .per_page(15);
    println!("✓ Orders paginator: page 2, 15 items per page, multiple filters");

    println!();
    println!("3. Return type demonstration:");
    println!("The execute() method returns: Result<ResponseListDto<T>, sqlx::Error>");
    println!("Where ResponseListDto<T> contains:");
    println!("  - meta: MetaResponseDto (page, per_page, total)");
    println!("  - data: Vec<T> (your struct data)");
    println!("  - message: String");
    println!("  - version: String");
    println!();

    println!("4. Usage in real application:");
    println!("```rust");
    println!("// In your handler/service:");
    println!("let pool = get_database_pool();");
    println!("let search_term = Some(\"john\".to_string());");
    println!();
    println!("let result: ResponseListDto<User> = paginator(\"users\")");
    println!("    .search(&[\"email\", \"fullname\"])");
    println!("    .filter_by(\"is_deleted\")");
    println!("    .filter(false)");
    println!("    .filter_by(\"status\")");
    println!("    .filter(\"active\")");
    println!("    .page(1)");
    println!("    .per_page(10)");
    println!("    .execute(&pool, search_term)");
    println!("    .await?;");
    println!();
    println!("// result.data contains Vec<User>");
    println!("// result.meta contains pagination info");
    println!("```");
    println!();

    println!("5. Builder pattern flexibility:");

    let minimal = paginator("users");
    println!("✓ Minimal: just table name (defaults: page=1, per_page=10)");

    let with_search = paginator("users").search(&["email"]);
    println!("✓ With search: searches in email column");

    let with_filters = paginator("users")
        .filter_by("active")
        .filter(true)
        .filter_by("role")
        .filter("admin");
    println!("✓ With filters: active=true AND role='admin'");

    let complete = paginator("users")
        .search(&["email", "fullname"])
        .filter_by("active")
        .filter(true)
        .page(3)
        .per_page(25);
    println!("✓ Complete: search + filter + pagination settings");

    println!();
    println!("=== All examples completed successfully! ===");
    println!("Note: To actually execute queries, you need a database connection pool.");
}
