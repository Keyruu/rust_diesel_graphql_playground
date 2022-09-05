use crate::schema::authors::dsl::authors as authorsDsl;
use crate::schema::posts;
use crate::Database;
use async_graphql::*;
use diesel::prelude::*;
use diesel::{Identifiable, QueryDsl, RunQueryDsl};

use super::authors::Author;

#[derive(SimpleObject, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Author))]
#[graphql(complex)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub author_id: i32,
}

#[ComplexObject]
impl Post {
    async fn author<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Author, Error> {
        let connection = &mut ctx.data::<Database>()?.pool.get()?;
        Ok(authorsDsl.find(self.author_id).first(connection)?)
    }
}
