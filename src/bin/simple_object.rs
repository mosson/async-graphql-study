use async_graphql::{
    ComplexObject, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject,
};

#[derive(SimpleObject)]
#[graphql(complex)]
#[allow(dead_code)]
struct MyObject {
    /// Value a docコメントはSDLに反映される
    a: i32,

    b: i32,

    #[graphql(skip)]
    c: i32,
}

#[ComplexObject]
impl MyObject {
    async fn e(&self) -> String {
        "Hello, calculated value".into()
    }
}

struct Query;

#[Object]
impl Query {
    // デフォルトでキャメルケースになる
    // カスタムの名前はマクロで登録可能
    #[graphql(name = "hoge")]
    async fn my_object(&self) -> MyObject {
        MyObject {
            a: 42,
            b: 24,
            c: 12,
        }
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    // スキーマを閲覧する
    println!("{}", schema.sdl());

    let res = schema.execute("{ hoge { a b e } }").await;
    assert!(res.errors.is_empty());
    dbg!(res);
    let res = schema.execute("{ hoge { a } }").await;
    assert!(res.errors.is_empty());
    dbg!(res);

    // エラーになる
    let res = schema.execute("{ hoge { c } }").await;
    assert!(!res.errors.is_empty());

    let res = schema.execute("{ hoge { d } }").await;
    assert!(!res.errors.is_empty());
}
