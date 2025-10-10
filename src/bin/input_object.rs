use async_graphql::{EmptyMutation, EmptySubscription, InputObject, Object, Schema, SimpleObject};

#[derive(InputObject)]
struct AddInput {
    a: i32,
    b: i32,
}

#[derive(SimpleObject)]
struct AddOutput {
    sum: i32,
}

struct Query;

#[Object]
impl Query {
    // クエリ中の名前はキャメルケースに変換したものが利用される
    async fn add(&self, add_input: AddInput) -> AddOutput {
        AddOutput {
            sum: add_input.a + add_input.b,
        }
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    println!("{}", schema.sdl());

    let res = schema
        .execute(
            r#"
                {
                    add(addInput: { a: 2, b: 3}) {
                        sum
                    }
                }
            "#,
        )
        .await;

    dbg!(res);
}
