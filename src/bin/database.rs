use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

async fn create_pool() -> Result<Pool<Sqlite>, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(":memory:")
        .await
}

struct Query;

#[Object]
impl Query {
    async fn forty_two(&self, ctx: &Context<'_>) -> async_graphql::Result<i32> {
        let db_pool = ctx.data::<Pool<Sqlite>>()?;

        let row: (i32,) = sqlx::query_as("SELECT 42").fetch_one(db_pool).await?;

        Ok(row.0)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = create_pool().await.map_err(|e| e.to_string())?;

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(db_pool)
        .finish();

    println!("{}", schema.sdl());

    let response = schema.execute("{ fortyTwo }").await;

    dbg!(&response);

    Ok(())
}
