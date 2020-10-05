use juniper::{GraphQLEnum, GraphQLObject, GraphQLInputObject};

#[derive(GraphQLEnum)]
pub enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct Human {
    pub id: String,
    pub name: String,
    pub appears_in: Vec<Episode>,
    pub home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct NewHuman {
    pub name: String,
    pub appears_in: Vec<Episode>,
    pub home_planet: String,
}