pub struct MutationRoot;
use juniper::FieldResult;
use super::Context;
use super::objects::{
    starwar::*,
};

#[juniper::object(Context = Context)]
/// This is the root for all kinds of mutations, if you want to change something in the schema, use this object
impl MutationRoot {
    /// Starwar mutation example
    fn create_human(new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
}