use async_graphql::{EmptyMutation, EmptySubscription, Object, Response, Schema};

struct Query;

#[Object]
impl Query {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let res = schema.execute("{ add(a: 10, b: 20) }").await;
    dbg!(&res);
    let Response { data: object, .. } = res;
    match object {
        async_graphql::Value::Object(map) => {
            match map.get("add".into()) {
                Some(async_graphql::Value::Number(v)) => {
                    println!("received value: {}", v);
                }
                _ => {} // noop
            }
        }
        _ => {} // noop
    }
}
