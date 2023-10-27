use std::net::SocketAddr;
use axum::{routing::get, Router};
use serde::Deserialize;

async fn index() -> &'static str {
    "asdfsadf"
}

async fn insert() -> &'static str {
    "inserting"
}

// review_table 
//      part_no 10 digits
//      pool_no
//      review_type
//      status
//      final_decision
//      created_time
//      last_updated

#[derive(Deserialize, Debug, Clone)]
pub enum ReviewType {
    Excuse,
    Disqualify,
}

#[derive(Deserialize, Debug, Clone)]
pub enum FinalDecision {
    Qualify,
    TemporaryExcuse,
    PermanentExcuse,
    Disqualify,
}

#[derive(Deserialize, Debug, Clone)]
pub enum ReviewStatus {
    Admin,
    Judge,
    Complete
}

#[derive(Deserialize, Debug, Clone)]
pub struct Timestamp {}

#[derive(Deserialize, Debug, Clone)]
pub struct ReviewRecord {
    pub part_no : u16,
    pub pool_no : u16,
    pub review_type : ReviewType,
    pub status : ReviewStatus,
    pub final_decision : FinalDecision,
    pub created : Timestamp,
    pub last_updated : Timestamp,
}

pub struct ReviewTable {
    results : Vec<ReviewRecord>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/",get(index))
        .route("/insert", get(insert));
    let addr = SocketAddr::from(([127,0,0,1], 3000));
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}