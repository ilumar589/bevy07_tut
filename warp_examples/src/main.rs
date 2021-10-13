use sqlx::postgres::{PgPoolOptions, PgConnectOptions};

mod migrations;
mod models;
mod handlers;
mod filters;
mod tests;

#[tokio::main]
async fn main() {
    dotenv::dotenv();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy_with(PgConnectOptions::new()
            .username("postgres")
            .password("postgres")
            .database("tests")
            .port(5438));

    // let migration = migrations::migrate(&pool).await; I don't know yet why this doesn't work, guess the sql name needs a specific format but the documentation doesn't mention it
    //
    // match migration {
    //     Err(error) => {
    //         panic!("{:?}", error);
    //     }
    //     _ => ()
    // }

    let api = filters::todos(pool);

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}
