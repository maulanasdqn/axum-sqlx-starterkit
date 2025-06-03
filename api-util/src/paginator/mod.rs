use crate::query_builder::SqlxParameterizedQueryBuilder;
use api_entity::{MetaResponseDto, ResponseListDto};
use serde::Serialize;
use sqlx::{Database, FromRow, Postgres, Row};
use std::fmt::Display;

pub struct Paginator {
    table: String,
    search_columns: Vec<String>,
    filters: Vec<(String, String)>,
    page: u64,
    per_page: u64,
}

impl Paginator {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            search_columns: Vec::new(),
            filters: vec![("is_deleted".to_string(), "false".to_string())],
            page: 1,
            per_page: 10,
        }
    }

    pub fn search(mut self, columns: &[&str]) -> Self {
        self.search_columns = columns.iter().map(|&s| s.to_string()).collect();
        self
    }

    pub fn filter_by(self, column: &str) -> PaginatorFilterBy {
        PaginatorFilterBy {
            paginator: self,
            column: column.to_string(),
        }
    }

    pub fn page(mut self, page: u64) -> Self {
        self.page = page;
        self
    }

    pub fn per_page(mut self, per_page: u64) -> Self {
        self.per_page = per_page;
        self
    }

    pub async fn execute<T>(
        self,
        executor: &sqlx::PgPool,
        search_term: Option<String>,
    ) -> Result<ResponseListDto<T>, sqlx::Error>
    where
        T: for<'r> FromRow<'r, <Postgres as Database>::Row> + Serialize + Send + Unpin,
    {
        let offset = (self.page - 1) * self.per_page;

        let mut query_builder = SqlxParameterizedQueryBuilder::new(&self.table);

        if !self.filters.is_empty() {
            for (column, value) in &self.filters {
                query_builder = query_builder.where_clause(column, value);
            }
        }

        if let Some(term) = search_term {
            if !self.search_columns.is_empty() {
                let mut search_query = String::new();
                for (i, column) in self.search_columns.iter().enumerate() {
                    if i > 0 {
                        search_query.push_str(" OR ");
                    }
                    search_query.push_str(&format!("{} ILIKE '%{}%'", column, term));
                }

                let mut base_query = query_builder.build_with_params().0;
                if base_query.contains("WHERE") {
                    base_query.push_str(&format!(" AND ({})", search_query));
                } else {
                    base_query.push_str(&format!(" WHERE ({})", search_query));
                }

                let count_query = format!(
                    "SELECT COUNT(*) FROM {} WHERE ({})",
                    self.table, search_query
                );
                let total_row = sqlx::query(&count_query).fetch_one(executor).await?;
                let total: i64 = total_row.get(0);

                let data_query =
                    format!("{} LIMIT {} OFFSET {}", base_query, self.per_page, offset);
                let rows = sqlx::query_as::<_, T>(&data_query)
                    .fetch_all(executor)
                    .await?;

                return Ok(ResponseListDto {
                    meta: MetaResponseDto {
                        page: Some(self.page),
                        per_page: Some(self.per_page),
                        total: Some(total as u64),
                    },
                    data: rows,
                    message: "Data retrieved successfully".to_string(),
                    version: "1.0.0".to_string(),
                });
            }
        }

        let count_query_builder = SqlxParameterizedQueryBuilder::new(&self.table);
        let mut count_query_builder = count_query_builder;

        if !self.filters.is_empty() {
            for (column, value) in &self.filters {
                count_query_builder = count_query_builder.where_clause(column, value);
            }
        }

        let (base_query, params) = count_query_builder.build_with_params();
        let count_query = base_query.replace("SELECT *", "SELECT COUNT(*)");

        let mut sqlx_count_query = sqlx::query(&count_query);
        for param in &params {
            sqlx_count_query = sqlx_count_query.bind(param);
        }

        let total_row = sqlx_count_query.fetch_one(executor).await?;
        let total: i64 = total_row.get(0);

        let data_query_builder = SqlxParameterizedQueryBuilder::new(&self.table);
        let mut data_query_builder = data_query_builder;

        if !self.filters.is_empty() {
            for (column, value) in &self.filters {
                data_query_builder = data_query_builder.where_clause(column, value);
            }
        }

        data_query_builder = data_query_builder.limit(self.per_page as i64);

        let (data_query, data_params) = data_query_builder.build_with_params();
        let final_data_query = format!("{} OFFSET {}", data_query, offset);

        let mut sqlx_data_query = sqlx::query_as::<_, T>(&final_data_query);
        for param in &data_params {
            sqlx_data_query = sqlx_data_query.bind(param);
        }
        sqlx_data_query = sqlx_data_query.bind(offset as i64);

        let rows = sqlx_data_query.fetch_all(executor).await?;

        Ok(ResponseListDto {
            meta: MetaResponseDto {
                page: Some(self.page),
                per_page: Some(self.per_page),
                total: Some(total as u64),
            },
            data: rows,
            message: "Data retrieved successfully".to_string(),
            version: "1.0.0".to_string(),
        })
    }
}

pub struct PaginatorFilterBy {
    paginator: Paginator,
    column: String,
}

impl PaginatorFilterBy {
    pub fn filter<T: Display>(mut self, value: T) -> Paginator {
        self.paginator
            .filters
            .push((self.column, value.to_string()));
        self.paginator
    }
}

pub fn paginator(table: &str) -> Paginator {
    Paginator::new(table)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;

    #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
    struct TestUser {
        pub id: i32,
        pub email: String,
        pub fullname: String,
        pub is_deleted: bool,
    }

    #[test]
    fn test_paginator_creation() {
        let pag = paginator("users");
        assert_eq!(pag.table, "users");
        assert_eq!(pag.page, 1);
        assert_eq!(pag.per_page, 10);
    }

    #[test]
    fn test_paginator_search_columns() {
        let pag = paginator("users").search(&["email", "fullname"]);
        assert_eq!(pag.search_columns, vec!["email", "fullname"]);
    }

    #[test]
    fn test_paginator_single_filter() {
        let pag = paginator("users")
            .filter_by("email")
            .filter("test@example.com");

        assert_eq!(pag.filters.len(), 2);
        assert_eq!(pag.filters[0].0, "is_deleted");
        assert_eq!(pag.filters[0].1, "false");
        assert_eq!(pag.filters[1].0, "email");
        assert_eq!(pag.filters[1].1, "test@example.com");
    }

    #[test]
    fn test_paginator_multiple_filters() {
        let pag = paginator("users")
            .filter_by("email")
            .filter("test@example.com")
            .filter_by("fullname")
            .filter("John Doe");

        assert_eq!(pag.filters.len(), 3);
        assert_eq!(pag.filters[0].0, "is_deleted");
        assert_eq!(pag.filters[0].1, "false");
        assert_eq!(pag.filters[1].0, "email");
        assert_eq!(pag.filters[1].1, "test@example.com");
        assert_eq!(pag.filters[2].0, "fullname");
        assert_eq!(pag.filters[2].1, "John Doe");
    }

    #[test]
    fn test_paginator_chaining() {
        let pag = paginator("users")
            .search(&["email", "fullname"])
            .filter_by("email")
            .filter("user-input-email")
            .filter_by("fullname")
            .filter("user-input-fullname")
            .page(2)
            .per_page(20);

        assert_eq!(pag.table, "users");
        assert_eq!(pag.search_columns, vec!["email", "fullname"]);
        assert_eq!(pag.filters.len(), 3);
        assert_eq!(pag.page, 2);
        assert_eq!(pag.per_page, 20);
    }

    #[test]
    fn test_paginator_builder_pattern() {
        let pag = paginator("users")
            .search(&["email", "fullname"])
            .filter_by("email")
            .filter("user-input-email")
            .filter_by("fullname")
            .filter("user-input-fullname");

        assert_eq!(pag.search_columns, vec!["email", "fullname"]);
        assert_eq!(
            pag.filters[0],
            ("is_deleted".to_string(), "false".to_string())
        );
        assert_eq!(
            pag.filters[1],
            ("email".to_string(), "user-input-email".to_string())
        );
        assert_eq!(
            pag.filters[2],
            ("fullname".to_string(), "user-input-fullname".to_string())
        );
    }

    #[test]
    fn test_paginator_page_settings() {
        let pag = paginator("users").page(5).per_page(25);

        assert_eq!(pag.page, 5);
        assert_eq!(pag.per_page, 25);
    }

    #[test]
    fn test_paginator_default_values() {
        let pag = paginator("products");
        assert_eq!(pag.page, 1);
        assert_eq!(pag.per_page, 10);
        assert!(pag.search_columns.is_empty());
        assert_eq!(pag.filters.len(), 1);
        assert_eq!(pag.filters[0].0, "is_deleted");
        assert_eq!(pag.filters[0].1, "false");
    }

    #[test]
    fn test_filter_by_returns_correct_type() {
        let filter_by = paginator("users").filter_by("status");
        assert_eq!(filter_by.column, "status");
    }

    #[test]
    fn test_complex_chaining_scenario() {
        let pag = paginator("orders")
            .search(&["customer_name", "order_id"])
            .filter_by("status")
            .filter("pending")
            .filter_by("created_date")
            .filter("2024-01-01")
            .page(3)
            .per_page(15);

        assert_eq!(pag.table, "orders");
        assert_eq!(pag.search_columns, vec!["customer_name", "order_id"]);
        assert_eq!(pag.filters.len(), 3);
        assert_eq!(
            pag.filters[0],
            ("is_deleted".to_string(), "false".to_string())
        );
        assert_eq!(
            pag.filters[1],
            ("status".to_string(), "pending".to_string())
        );
        assert_eq!(
            pag.filters[2],
            ("created_date".to_string(), "2024-01-01".to_string())
        );
        assert_eq!(pag.page, 3);
        assert_eq!(pag.per_page, 15);
    }
}
