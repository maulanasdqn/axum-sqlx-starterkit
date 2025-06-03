use crate::MetaResponseDto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResponseMessageDto {
    pub message: String,
    pub version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ResponseDetailDto<T: Serialize> {
    pub message: String,
    pub data: T,
    pub version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ResponseListDto<T: Serialize> {
    pub meta: MetaResponseDto,
    pub data: Vec<T>,
    pub message: String,
    pub version: String,
}
