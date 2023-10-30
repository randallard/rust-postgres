use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    model::{JuryModel},
    schema::{CreateJurySchema, UpdateJurySchema, FilterOptions},
    AppState,
};

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Healthy";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}


// Jury Admin
pub async fn jury_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        JuryModel,
        "SELECT * FROM jury ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all jury items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let jury = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": jury.len(),
        "jury": jury
    });
    Ok(Json(json_response))
}

pub async fn create_jury_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateJurySchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        JuryModel,
        "INSERT INTO jury (part_no, pool_no, review_type, _status, final_decision) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        body.part_no.to_string(),
        body.pool_no.to_string(),
        body.review_type.to_string(),
        body._status.to_string(),
        body.final_decision.to_string()
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(jury) => {
            let jury_response = json!({"status": "success","data": json!({
                "jury": jury
            })});

            return Ok((StatusCode::CREATED, Json(jury_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "jury with that part_no already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}


pub async fn get_jury_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(JuryModel, "SELECT * FROM jury WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(jury) => {
            let jury_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "jury": jury
            })});

            return Ok(Json(jury_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Jury with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

pub async fn edit_jury_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateJurySchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(JuryModel, "SELECT * FROM jury WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Jury with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let jury = query_result.unwrap();

    let query_result = sqlx::query_as!(
        JuryModel,
        "UPDATE jury SET part_no = $1, pool_no = $2, review_type = $3, _status = $4, final_decision = $5, updated_at = $6 WHERE id = $7 RETURNING *",
        body.part_no.to_owned().unwrap_or(jury.part_no),
        body.pool_no.to_owned().unwrap_or(jury.pool_no),
        body.review_type.to_owned().unwrap_or(jury.review_type),
        body._status.to_owned().unwrap_or(jury._status),
        body.final_decision.to_owned().unwrap_or(jury.final_decision),
        now,
        id
    )
    .fetch_one(&data.db)
    .await
    ;

    match query_result {
        Ok(jury) => {
            let jury_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "jury": jury
            })});

            return Ok(Json(jury_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}

pub async fn delete_jury_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!("DELETE FROM jury  WHERE id = $1", id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Jury with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}