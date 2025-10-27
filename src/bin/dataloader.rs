use async_graphql::{
    ComplexObject, Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject,
};
use sqlx::{Pool, Sqlite, SqlitePool, prelude::FromRow, sqlite::SqlitePoolOptions};
use tracing_subscriber::EnvFilter;

// SimpleObject で型定義を生成してGQLクライアントで扱えるようにする
// FromRow で sqlx から生成できるようにする
#[derive(SimpleObject, FromRow)]
struct User {
    id: u32,
    todo_id: u32,
}

// complex で メソッドをプロパティとして扱えるようにする
#[derive(SimpleObject, FromRow)]
#[graphql(complex)]
struct Todo {
    id: u32,
}

#[ComplexObject]
impl Todo {
    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let pool = ctx.data_unchecked::<Pool<Sqlite>>();

        let users =
            sqlx::query_as::<_, User>(r#"SELECT id, todo_id FROM users WHERE todo_id = ?1"#)
                .bind(self.id)
                .fetch_all(pool)
                .await?;

        Ok(users)
    }
}

struct Query;

#[Object]
impl Query {
    async fn todos(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Todo>> {
        let pool = ctx.data_unchecked::<Pool<Sqlite>>();

        let todos = sqlx::query_as::<_, Todo>(r#"SELECT id FROM todos"#)
            .fetch_all(pool)
            .await?;

        Ok(todos)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // sqlx で発行されたクエリをログする
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("sqlx=trace".parse().unwrap()))
        .init();

    let db_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(":memory:")
        .await
        .map_err(|e| e.to_string())?;

    // テスト用データ投入
    setup(&db_pool).await.map_err(|e| e.to_string())?;

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(db_pool)
        .finish();

    let response = schema
        .execute(
            r#"
                query {
                    todos {
                        id,
                        users {
                            id
                        }
                    }
                }
            "#,
        )
        .await;
    dbg!(&response);

    Ok(())
}

async fn setup(db_pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id INT NOT NULL PRIMARY KEY
        );
    "#,
    )
    .execute(db_pool)
    .await?;
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INT NOT NULL PRIMARY KEY,
            todo_id INT NOT NULL
        );
    "#,
    )
    .execute(db_pool)
    .await?;
    sqlx::query(r#"INSERT INTO todos (id) VALUES (1);"#)
        .execute(db_pool)
        .await?;
    sqlx::query(r#"INSERT INTO todos (id) VALUES (2);"#)
        .execute(db_pool)
        .await?;
    sqlx::query(r#"INSERT INTO todos (id) VALUES (3);"#)
        .execute(db_pool)
        .await?;
    sqlx::query(r#"INSERT INTO users (id, todo_id) VALUES (11, 1);"#)
        .execute(db_pool)
        .await?;
    sqlx::query(r#"INSERT INTO users (id, todo_id) VALUES (12, 1);"#)
        .execute(db_pool)
        .await?;
    sqlx::query(r#"INSERT INTO users (id, todo_id) VALUES (13, 1);"#)
        .execute(db_pool)
        .await?;
    sqlx::query(r#"INSERT INTO users (id, todo_id) VALUES (14, 2);"#)
        .execute(db_pool)
        .await?;
    sqlx::query(r#"INSERT INTO users (id, todo_id) VALUES (15, 2);"#)
        .execute(db_pool)
        .await?;
    sqlx::query(r#"INSERT INTO users (id, todo_id) VALUES (16, 3);"#)
        .execute(db_pool)
        .await?;
    Ok(())
}
