use crate::models::{ListOptions, Todo, TodoId};
use std::convert::Infallible;
use warp::http::StatusCode;
use sqlx::{PgPool, Row, Transaction, Error, Postgres};
use sqlx::postgres::PgRow;

/// These are our API handlers, the ends of each filter chain.
/// Notice how thanks to using `Filter::and`, we can define a function
/// with the exact arguments we'd expect from each filter in the chain.
/// No tuples are needed, it's auto flattened for the functions.

pub async fn list_todos(opts: ListOptions, pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    // just return a json array of todos, applying the limit and offset

    let todos = sqlx::query_as::<_, Todo>("select * from todos").fetch_all(&pool).await.unwrap();

    Ok(warp::reply::json(&todos))
}

pub async fn create_todo(create: Todo, pool: PgPool) -> Result<Box<dyn warp::Reply>, Infallible> {
    let todo_ids = sqlx::query_as!(TodoId,"select id from todos").fetch_all(&pool).await.unwrap();

    for todo in todo_ids.iter() {
        if todo.id == create.id {
            return Ok(Box::new(StatusCode::BAD_REQUEST));
        }
    }

    let transaction = pool.begin().await;

    let transaction = match transaction {
        Ok(tr) => tr,
        Err(error) => panic!("Error while getting transaction {:?}", error)
    };

    let new_todo = sqlx::query_as!(Todo,
      r#"insert into todos(text,completed)
      values ($1,$2)
      returning id, text, completed"#,
      &create.text,
      &create.completed)
        .fetch_one(&pool).await;

    let new_todo = match new_todo {
        Ok(created_todo) => {
            transaction.commit().await.unwrap();
            created_todo
        },
        Err(error) => {
            transaction.rollback().await.unwrap();
            panic!("Could not create a new todo {:?}", error); // should not panic but provide appropriate responses
        }
    };


    Ok(Box::new(warp::reply::with_status(warp::reply::json(&new_todo),
                                         StatusCode::CREATED)))
}

// pub async fn update_tod(id: u64, update: Todo, pool: &PgPool) -> Result<impl warp::Reply, Infallible> {
//
//     for todo in vec.iter_mut() {
//         if todo.id == id {
//             *todo = update;
//             return Ok(StatusCode::OK);
//         }
//     }
//
//     Ok(StatusCode::NOT_FOUND)
// }
//
// pub async fn delete_todo(id: u64, pool: &PgPool) -> Result<impl warp::Reply, Infallible> {
//     let mut vec = db.lock().await;
//
//     let len = vec.len();
//     vec.retain(|todo| {
//         todo.id != id
//     });
//
//     let deleted = vec.len() != len;
//
//     if deleted {
//         Ok(StatusCode::NO_CONTENT)
//     } else {
//         Ok(StatusCode::NOT_FOUND)
//     }
// }