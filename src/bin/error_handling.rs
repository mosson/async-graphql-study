use async_graphql::{EmptyMutation, EmptySubscription, ErrorExtensions, Object, Schema};

struct Query;

#[Object]
impl Query {
    // Error は std::fmt::Display を実装していれば良い
    async fn foo(&self) -> async_graphql::Result<String> {
        Err("This is Error".into())
    }

    async fn bar(&self) -> async_graphql::Result<i32> {
        Ok("234a".parse::<i32>().map_err(|e| {
            // ErrorExtensions によって標準エラーが拡張されている
            e.extend_with(|_, e| {
                e.set("code", 400);
            })
        })?)
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    println!("{}", schema.sdl());

    dbg!(schema.execute("{ foo }").await);
    // Response {
    //     data: Null,
    //     extensions: {},
    //     cache_control: CacheControl {
    //         public: true,
    //         max_age: 0,
    //     },
    //     errors: [
    //         ServerError {
    //             message: "This is Error",
    //             locations: [
    //                 Pos(1:3),
    //             ],
    //             path: [
    //                 Field(
    //                     "foo",
    //                 ),
    //             ],
    //             extensions: None,
    //         },
    //     ],
    //     http_headers: {},
    // }

    dbg!(schema.execute("{ bar }").await);
}
