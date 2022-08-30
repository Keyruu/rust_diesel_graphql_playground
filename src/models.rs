use crate::schema::authors::dsl::authors as authorsDsl;
use crate::schema::posts::author_id;
use crate::schema::posts::dsl::posts as postsDsl;
use crate::schema::{authors, posts};
use crate::Database;
use async_graphql::*;
use diesel::prelude::*;
use diesel::{Identifiable, QueryDsl, RunQueryDsl};

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
    async fn author<'ctx>(&self, ctx: &Context<'ctx>) -> Author {
        let connection = &mut ctx.data::<Database>().unwrap().pool.get().unwrap();
        authorsDsl
            .find(self.author_id)
            .first(connection)
            .expect("No Author found.")
    }
}

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
