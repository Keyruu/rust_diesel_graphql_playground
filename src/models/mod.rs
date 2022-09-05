use crate::schema::authors::dsl::authors as authorsDsl;
use crate::schema::authors::table as authorsTable;
use crate::schema::posts::dsl::posts as postsDsl;
use async_graphql::{connection, Context, Data, Error, Object};
mod authors;
mod posts;
use async_graphql::*;
use authors::*;
use diesel::{associations::HasTable, RunQueryDsl};
use posts::*;

use crate::Database;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn posts<'a>(&self, ctx: &Context<'a>) -> Result<Vec<Post>, Error> {
        let connection = &mut ctx.data::<Database>()?.pool.get()?;
        Ok(postsDsl.load::<Post>(connection)?)
    }

    async fn authors<'a>(&self, ctx: &Context<'a>) -> Result<Vec<Author>, Error> {
        let connection = &mut ctx.data::<Database>()?.pool.get()?;
        Ok(authorsDsl.load::<Author>(connection)?)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add_author<'a>(&self, ctx: &Context<'a>, name: String) -> Result<Author, Error> {
        let connection = &mut ctx.data::<Database>()?.pool.get()?;

        let author = AuthorInput { name };

        Ok(diesel::insert_into(authorsTable)
            .values(&author)
            .get_result::<Author>(connection)?)
    }
}
