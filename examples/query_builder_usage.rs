use api_util::{parameterized_query_builder, query_builder, QueryBuilderTrait};

fn main() {
    println!("=== SQLx Query Builder Examples ===\n");

    println!("1. Basic usage with the exact syntax requested:");
    let query = query_builder("users")
        .select(&["email", "fullname"])
        .where_clause("is_deleted", false)
        .limit(10)
        .build();
    println!("Query: {}\n", query);

    println!("2. Select all columns:");
    let query = query_builder("products").build();
    println!("Query: {}\n", query);

    println!("3. Multiple where clauses:");
    let query = query_builder("orders")
        .select(&["id", "customer_id", "total"])
        .where_clause("status", "pending")
        .where_clause("created_at", "2024-01-01")
        .limit(50)
        .build();
    println!("Query: {}\n", query);

    println!("4. Using trait for generic functions:");
    fn build_active_records<T: QueryBuilderTrait>(builder: T) -> String {
        builder
            .select(&["id", "name", "status"])
            .where_clause("active", true)
            .limit(20)
            .build()
    }

    let query = build_active_records(query_builder("categories"));
    println!("Query: {}\n", query);

    println!("5. Parameterized query (SQL injection safe):");
    let (query, params) = parameterized_query_builder("users")
        .select(&["email", "fullname"])
        .where_clause("is_deleted", false)
        .where_clause("role", "admin")
        .limit(10)
        .build_with_params();
    println!("Query: {}", query);
    println!("Parameters: {:?}\n", params);

    println!("6. Building SQLx QueryBuilder for direct database execution:");
    let sqlx_query = parameterized_query_builder("users")
        .select(&["id", "email"])
        .where_clause("active", true)
        .limit(5)
        .build_sqlx_query();
    println!("SQLx QueryBuilder created successfully for database execution\n");

    println!("=== All examples completed successfully! ===");
}
