use crate::schema::authors::dsl::authors as authorsDsl;
use crate::schema::posts::dsl::posts as postsDsl;
use async_graphql::{Context, Object};
mod authors;
mod posts;
use authors::*;
use diesel::RunQueryDsl;
use posts::*;

use crate::Database;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn posts<'a>(&self, ctx: &Context<'a>) -> Vec<Post> {
        let connection = &mut ctx.data::<Database>().unwrap().pool.get().unwrap();
        postsDsl
            .load::<Post>(connection)
            .expect("Error loading posts")
    }

    async fn authors<'a>(&self, ctx: &Context<'a>) -> Vec<Author> {
        let connection = &mut ctx.data::<Database>().unwrap().pool.get().unwrap();
        authorsDsl
            .load::<Author>(connection)
            .expect("Error loading authors")
    }
}
