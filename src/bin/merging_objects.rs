use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Object, Schema};

struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self) -> Vec<String> {
        vec!["John".to_string(), "Smith".to_string(), "Ora".to_string()]
    }
}

struct MovieQuery;

#[Object]
impl MovieQuery {
    async fn movies(&self) -> Vec<String> {
        vec!["A".to_string(), "B".to_string(), "C".to_string()]
    }
}

#[derive(MergedObject)]
struct Query(UserQuery, MovieQuery);

#[tokio::main]
async fn main() {
    let schema = Schema::build(
        Query(UserQuery, MovieQuery),
        EmptyMutation,
        EmptySubscription,
    )
    .finish();

    println!("{}", schema.sdl());

    println!("\n-----------\n");

    dbg!(schema.execute("{ users }").await);

    println!("\n-----------\n");

    dbg!(schema.execute("{ movies }").await);
}
