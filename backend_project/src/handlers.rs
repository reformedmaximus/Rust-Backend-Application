use crate::models::CustomDetail;
use axum::{extract::Extension, http::StatusCode, Json};
use std::sync::Arc;
use tokio_postgres::Client;

pub async fn create_custom_detail(
    Json(payload): Json<CustomDetail>, // this lines validates the incoming json payload against the customdetail structure
    Extension(pool): Extension<Arc<Client>>,
) -> Result<StatusCode, StatusCode> {
    let stmt = pool
        .prepare("INSERT INTO custom_details (name, email) VALUES ($1, $2)")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    pool.execute(&stmt, &[&payload.name, &payload.email])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::CREATED) //returns the result of the operation with a status code 
}

pub async fn get_custom_details(
  Extension(pool): Extension<Arc<Client>>,
) -> Result<Json<Vec<CustomDetail>>, StatusCode> {
  //fetch data from db 
  let stmt = pool.prepare("SELECT name, email FROM custom_details")
      .await
      .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

  let rows = pool.query(&stmt, &[])
      .await
      .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

  // convert data from serialized format (json) to rust data structure with deserialize 
  let details: Vec<CustomDetail> = rows.iter()
      .map(|row| CustomDetail {
          name: row.get(0),
          email: row.get(1),
      })
      .collect();

  Ok(Json(details)) //returns data in json format 
}

