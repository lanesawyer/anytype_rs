//! Objects module
//!
//! Handles object management operations.

use super::AnytypeClient;
use crate::{error::Result, types::Pagination};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Object information
#[derive(Debug, Deserialize, Serialize)]
pub struct Object {
    pub id: String,
    pub name: Option<String>,
    pub space_id: Option<String>,
    pub object: Option<String>, // object type
    pub properties: serde_json::Value,
    pub markdown: Option<String>,
    // Add more fields as needed
}

/// Response for listing objects
#[derive(Debug, Deserialize)]
pub struct ListObjectsResponse {
    pub data: Vec<Object>,
    pub pagination: Pagination,
}

/// Response for getting an object
#[derive(Debug, Deserialize)]
pub struct GetObjectResponse {
    pub object: Object,
}


/// Request to create a new object
#[derive(Debug, Serialize)]
pub struct CreateObjectRequest {
    pub type_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

/// Response when creating an object
#[derive(Debug, Deserialize)]
pub struct CreateObjectResponse {
    pub object: Object,
    pub properties: Option<serde_json::Value>,
    pub markdown: Option<String>,
}

/// Response when deleting an object
#[derive(Debug, Deserialize)]
pub struct DeleteObjectResponse {
    pub object: Object,
}

/// Request to update an existing object
#[derive(Debug, Serialize)]
pub struct UpdateObjectRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

/// Response when updating an object
#[derive(Debug, Deserialize)]
pub struct UpdateObjectResponse {
    pub object: Object,
    pub properties: Option<serde_json::Value>,
    pub markdown: Option<String>,
}

impl AnytypeClient {
    /// List objects in a space
    pub async fn list_objects(&self, space_id: &str) -> Result<Vec<Object>> {
        let response: ListObjectsResponse =
            self.get(&format!("/v1/spaces/{space_id}/objects")).await?;
        Ok(response.data)
    }

    /// Get a specific object by ID
    pub async fn get_object(&self, space_id: &str, object_id: &str) -> Result<GetObjectResponse> {
        self.get(&format!("/v1/spaces/{space_id}/objects/{object_id}"))
            .await
    }

    /// Create a new object in a space
    pub async fn create_object(
        &self,
        space_id: &str,
        request: CreateObjectRequest,
    ) -> Result<CreateObjectResponse> {
        info!("Creating object in space: {}", space_id);
        debug!("Request: {:?}", request);
        debug!("Request JSON: {}", serde_json::to_string_pretty(&request)?);

        self.post(&format!("/v1/spaces/{space_id}/objects"), &request)
            .await
    }

    /// Delete an object in a space (marks it as archived)
    pub async fn delete_object(
        &self,
        space_id: &str,
        object_id: &str,
    ) -> Result<DeleteObjectResponse> {
        info!("Deleting object {} in space: {}", object_id, space_id);

        self.delete(&format!("/v1/spaces/{space_id}/objects/{object_id}"))
            .await
    }

    /// Update an existing object in a space
    pub async fn update_object(
        &self,
        space_id: &str,
        object_id: &str,
        request: UpdateObjectRequest,
    ) -> Result<UpdateObjectResponse> {
        info!("Updating object {} in space: {}", object_id, space_id);
        debug!("Request: {:?}", request);
        debug!("Request JSON: {}", serde_json::to_string_pretty(&request)?);

        self.patch(
            &format!("/v1/spaces/{space_id}/objects/{object_id}"),
            &request,
        )
        .await
    }

    /// List objects in a space with pagination information
    pub async fn list_objects_with_pagination(
        &self,
        space_id: &str,
    ) -> Result<ListObjectsResponse> {
        self.get(&format!("/v1/spaces/{space_id}/objects")).await
    }
}
