use self::models::*;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptyMutation, EmptySubscription, Object, Schema,
};
use async_graphql_poem::GraphQL;
use cargo_test::schema::authors::dsl::authors;
use cargo_test::schema::posts::dsl::posts;
use cargo_test::*;

use diesel::prelude::*;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn posts<'a>(&self, ctx: &Context<'a>) -> Vec<Post> {
        let connection = &mut ctx.data::<Database>().unwrap().pool.get().unwrap();
        posts.load::<Post>(connection).expect("Error loading posts")
    }

    async fn authors<'a>(&self, ctx: &Context<'a>) -> Vec<Author> {
        let connection = &mut ctx.data::<Database>().unwrap().pool.get().unwrap();
        authors
            .load::<Author>(connection)
            .expect("Error loading authors")
    }
}

#[tokio::main]
async fn main() {
    let connection = get_connection_pool();

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(Database { pool: connection })
        .finish();

    let app = Route::new().at("/", get(graphql_playground).post(GraphQL::new(schema)));

    println!("Playground: http://localhost:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
        .unwrap();
}
