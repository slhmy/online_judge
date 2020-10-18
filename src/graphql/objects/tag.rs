use juniper::GraphQLObject;
use crate::graphql::objects::problem::*;

#[derive(GraphQLObject)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub related_problems: Vec<Problem>,
}