use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        health_checker_handler, 

        get_jury_handler,
        create_jury_handler, 
        delete_jury_handler, 
        edit_jury_handler,
        jury_list_handler

    },
    AppState,
};

async fn index() -> &'static str {
    "Welcome to the Jury API"
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/api/healthchecker", get(health_checker_handler))

        .route("/api/jury/", post(create_jury_handler))
        .route("/api/jury", get(jury_list_handler))
        .route(
            "/api/jury/:id",
             get(get_jury_handler)
            .patch(edit_jury_handler)
            .delete(delete_jury_handler),
        )
        .with_state(app_state)
}
