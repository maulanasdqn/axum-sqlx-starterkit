use sqlx::{Postgres, QueryBuilder};
use std::fmt::Display;

pub trait QueryBuilderTrait {
    fn select(self, columns: &[&str]) -> Self;
    fn where_clause<T: Display>(self, column: &str, value: T) -> Self;
    fn limit(self, limit: i64) -> Self;
    fn build(self) -> String;
}

pub struct SqlxQueryBuilder {
    table: String,
    select_columns: Vec<String>,
    where_clauses: Vec<String>,
    limit_value: Option<i64>,
}

impl SqlxQueryBuilder {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            select_columns: Vec::new(),
            where_clauses: vec!["is_deleted = 'false'".to_string()],
            limit_value: None,
        }
    }
}

impl QueryBuilderTrait for SqlxQueryBuilder {
    fn select(mut self, columns: &[&str]) -> Self {
        self.select_columns = columns.iter().map(|&s| s.to_string()).collect();
        self
    }

    fn where_clause<T: Display>(mut self, column: &str, value: T) -> Self {
        let clause = format!("{} = '{}'", column, value);
        self.where_clauses.push(clause);
        self
    }

    fn limit(mut self, limit: i64) -> Self {
        self.limit_value = Some(limit);
        self
    }

    fn build(self) -> String {
        let mut query = String::new();

        query.push_str("SELECT ");
        if self.select_columns.is_empty() {
            query.push('*');
        } else {
            query.push_str(&self.select_columns.join(", "));
        }

        query.push_str(&format!(" FROM {}", self.table));

        if !self.where_clauses.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.where_clauses.join(" AND "));
        }

        if let Some(limit) = self.limit_value {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        query
    }
}

pub fn query_builder(table: &str) -> SqlxQueryBuilder {
    SqlxQueryBuilder::new(table)
}

pub struct SqlxParameterizedQueryBuilder {
    table: String,
    select_columns: Vec<String>,
    where_clauses: Vec<String>,
    parameters: Vec<String>,
    limit_value: Option<i64>,
}

impl SqlxParameterizedQueryBuilder {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            select_columns: Vec::new(),
            where_clauses: Vec::new(),
            parameters: Vec::new(),
            limit_value: None,
        }
    }

    pub fn select(mut self, columns: &[&str]) -> Self {
        self.select_columns = columns.iter().map(|&s| s.to_string()).collect();
        self
    }

    pub fn where_clause<T: Display>(mut self, column: &str, value: T) -> Self {
        let param_index = self.parameters.len() + 1;
        let clause = format!("{} = ${}", column, param_index);
        self.where_clauses.push(clause);
        self.parameters.push(value.to_string());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit_value = Some(limit);
        self
    }

    pub fn build_with_params(self) -> (String, Vec<String>) {
        let mut query = String::new();

        query.push_str("SELECT ");
        if self.select_columns.is_empty() {
            query.push('*');
        } else {
            query.push_str(&self.select_columns.join(", "));
        }

        query.push_str(&format!(" FROM {}", self.table));

        if !self.where_clauses.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.where_clauses.join(" AND "));
        }

        if let Some(limit) = self.limit_value {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        (query, self.parameters)
    }

    pub fn build_sqlx_query<'a>(self) -> QueryBuilder<'a, Postgres> {
        let mut query_builder = QueryBuilder::new("SELECT ");

        if self.select_columns.is_empty() {
            query_builder.push("*");
        } else {
            for (i, column) in self.select_columns.iter().enumerate() {
                if i > 0 {
                    query_builder.push(", ");
                }
                query_builder.push(&column);
            }
        }

        query_builder.push(" FROM ");
        query_builder.push(&self.table);

        if !self.where_clauses.is_empty() {
            query_builder.push(" WHERE ");
            for (i, clause) in self.where_clauses.iter().enumerate() {
                if i > 0 {
                    query_builder.push(" AND ");
                }
                let parts: Vec<&str> = clause.split(" = ").collect();
                if parts.len() == 2 {
                    query_builder.push(parts[0]);
                    query_builder.push(" = ");
                    let param_index: usize = parts[1].trim_start_matches('$').parse().unwrap_or(1);
                    if param_index <= self.parameters.len() {
                        query_builder.push_bind(self.parameters[param_index - 1].clone());
                    }
                }
            }
        }

        if let Some(limit) = self.limit_value {
            query_builder.push(" LIMIT ");
            query_builder.push_bind(limit);
        }

        query_builder
    }
}

pub fn parameterized_query_builder(table: &str) -> SqlxParameterizedQueryBuilder {
    SqlxParameterizedQueryBuilder::new(table)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_select_all() {
        let query = query_builder("users").build();
        assert_eq!(query, "SELECT * FROM users WHERE is_deleted = 'false'");
    }

    #[test]
    fn test_select_specific_columns() {
        let query = query_builder("users")
            .select(&["email", "fullname"])
            .build();
        assert_eq!(
            query,
            "SELECT email, fullname FROM users WHERE is_deleted = 'false'"
        );
    }

    #[test]
    fn test_select_with_where_clause() {
        let query = query_builder("users")
            .select(&["email", "fullname"])
            .where_clause("is_deleted", false)
            .build();
        assert_eq!(
            query,
            "SELECT email, fullname FROM users WHERE is_deleted = 'false' AND is_deleted = 'false'"
        );
    }

    #[test]
    fn test_select_with_where_and_limit() {
        let query = query_builder("users")
            .select(&["email", "fullname"])
            .where_clause("is_deleted", false)
            .limit(10)
            .build();
        assert_eq!(
            query,
            "SELECT email, fullname FROM users WHERE is_deleted = 'false' AND is_deleted = 'false' LIMIT 10"
        );
    }

    #[test]
    fn test_multiple_where_clauses() {
        let query = query_builder("users")
            .select(&["email", "fullname"])
            .where_clause("is_deleted", false)
            .where_clause("status", "active")
            .limit(10)
            .build();
        assert_eq!(
            query,
            "SELECT email, fullname FROM users WHERE is_deleted = 'false' AND is_deleted = 'false' AND status = 'active' LIMIT 10"
        );
    }

    #[test]
    fn test_where_with_numeric_value() {
        let query = query_builder("users")
            .select(&["email", "fullname"])
            .where_clause("age", 25)
            .build();
        assert_eq!(
            query,
            "SELECT email, fullname FROM users WHERE is_deleted = 'false' AND age = '25'"
        );
    }

    #[test]
    fn test_parameterized_query_builder() {
        let (query, params) = parameterized_query_builder("users")
            .select(&["email", "fullname"])
            .where_clause("is_deleted", false)
            .limit(10)
            .build_with_params();

        assert_eq!(
            query,
            "SELECT email, fullname FROM users WHERE is_deleted = $1 LIMIT 10"
        );
        assert_eq!(params, vec!["false"]);
    }

    #[test]
    fn test_parameterized_multiple_where() {
        let (query, params) = parameterized_query_builder("users")
            .select(&["email", "fullname"])
            .where_clause("is_deleted", false)
            .where_clause("status", "active")
            .build_with_params();

        assert_eq!(
            query,
            "SELECT email, fullname FROM users WHERE is_deleted = $1 AND status = $2"
        );
        assert_eq!(params, vec!["false", "active"]);
    }

    #[test]
    fn test_trait_implementation() {
        fn use_query_builder<T: QueryBuilderTrait>(builder: T) -> String {
            builder
                .select(&["id", "name"])
                .where_clause("active", true)
                .limit(5)
                .build()
        }

        let query = use_query_builder(query_builder("products"));
        assert_eq!(
            query,
            "SELECT id, name FROM products WHERE is_deleted = 'false' AND active = 'true' LIMIT 5"
        );
    }

    #[test]
    fn test_empty_select_defaults_to_star() {
        let query = query_builder("users").where_clause("id", 1).build();
        assert_eq!(
            query,
            "SELECT * FROM users WHERE is_deleted = 'false' AND id = '1'"
        );
    }

    #[test]
    fn test_only_limit() {
        let query = query_builder("users").limit(100).build();
        assert_eq!(
            query,
            "SELECT * FROM users WHERE is_deleted = 'false' LIMIT 100"
        );
    }

    #[test]
    fn test_builder_pattern_chaining() {
        let builder = query_builder("orders");
        let query = builder
            .select(&["order_id", "customer_id", "total"])
            .where_clause("status", "pending")
            .where_clause("created_at", "2024-01-01")
            .limit(50)
            .build();

        assert_eq!(
            query,
            "SELECT order_id, customer_id, total FROM orders WHERE is_deleted = 'false' AND status = 'pending' AND created_at = '2024-01-01' LIMIT 50"
        );
    }
}
