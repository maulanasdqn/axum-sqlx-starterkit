use anyhow::Result;
use api_entity::{AppState, MetaRequestDto, ResponseListDto, TableEnum};
use api_util::paginator;

use crate::PermissionItemDto;

pub struct PermissionsRepository {
    pool: AppState,
}

impl PermissionsRepository {
    pub fn new(pool: AppState) -> Self {
        Self { pool }
    }

    pub async fn query_list(
        &self,
        meta: MetaRequestDto,
    ) -> Result<ResponseListDto<PermissionItemDto>> {
        let table_name = TableEnum::Permissions.to_string();
        let result = paginator(&table_name)
            .search(&["name"])
            .page(meta.page.unwrap_or(1))
            .per_page(meta.per_page.unwrap_or(10))
            .execute::<PermissionItemDto>(&self.pool, meta.search)
            .await?;
        Ok(result)
    }
}
