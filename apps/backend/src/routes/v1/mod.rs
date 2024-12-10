use axum::Router;

mod users;

pub fn routes() -> Router {
    Router::new()
        .merge(users::routes())
}
