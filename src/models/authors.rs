use crate::schema::authors;
use crate::Database;
use async_graphql::*;
use diesel::prelude::*;
use diesel::{Identifiable, RunQueryDsl};

use super::posts::Post;

#[derive(SimpleObject, Queryable, Identifiable, Insertable)]
#[graphql(complex)]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[ComplexObject]
impl Author {
    async fn posts<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Post>, Error> {
        let connection = &mut ctx.data::<Database>()?.pool.get()?;
        Ok(Post::belonging_to(&self).load(connection)?)
    }
}

#[derive(Insertable, InputObject)]
#[table_name = "authors"]
pub struct AuthorInput {
    pub name: String,
}
