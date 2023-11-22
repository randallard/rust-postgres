use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateJurySchema {
    pub part_no: String,
    pub pool_no: String,
    pub review_type: String,
    pub _status: String,
    pub final_decision: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateJurySchema {
    pub part_no: Option<String>,
    pub pool_no: Option<String>,
    pub review_type: Option<String>,
    pub _status: Option<String>,
    pub final_decision: Option<String>,
}

