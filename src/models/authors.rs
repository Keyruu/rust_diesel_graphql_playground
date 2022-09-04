use crate::schema::authors;
use crate::Database;
use async_graphql::*;
use diesel::prelude::*;
use diesel::{Identifiable, RunQueryDsl};

use super::posts::Post;

#[derive(SimpleObject, Queryable, Identifiable)]
#[graphql(complex)]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[ComplexObject]
impl Author {
    async fn posts<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Post> {
        let connection = &mut ctx.data::<Database>().unwrap().pool.get().unwrap();

        Post::belonging_to(&self)
            .load(connection)
            .expect("Error loading photos")
    }
}
