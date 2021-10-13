use crate::models::{ListOptions, Todo};
use warp::Filter;
use super::handlers;
use sqlx::PgPool;

/// the 4 todos filters combined
pub fn todos(pool: PgPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    todos_list(pool.clone())
        .or(todos_create(pool.clone()))

    //todo complete with update or create, don't care right now, the principle is the same
}


/// GET /todos?offset=3&limit=5
pub fn todos_list(pool: PgPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todos")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_db(pool))
        .and_then(handlers::list_todos)
}

/// POST /todos with json body
pub fn todos_create(pool: PgPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todos")
        .and(warp::post())
        .and(json_body())
        .and(with_db(pool))
        .and_then(handlers::create_todo)
}

fn with_db(pool: PgPool) -> impl Filter<Extract = (PgPool,), Error=std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

fn json_body() -> impl Filter<Extract = (Todo,), Error=warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}