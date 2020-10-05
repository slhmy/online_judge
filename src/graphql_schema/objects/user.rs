use juniper::{GraphQLEnum, GraphQLObject};
use super::status::*;

#[derive(GraphQLObject)]
/// Inner object for 'User'.
/// It will be used to store properties user create in online-judge
/// Properties are like: satus, virtual currencies etc.
pub struct UserProperty {
    pub status: Vec<Status>,
}

#[derive(GraphQLEnum)]
/// Inner enum_type for 'User'.
/// 'Identity' is the basis for event authentication.
/// It will represent an level in authentication settings. 
pub enum UserIdentity {
    NetFriend,
    Student,
    Teacher, 
    Admin,
}

#[derive(GraphQLObject)]
/// Inner object for 'User'.
/// It will be used to store several kinds of personal information if the user set them.
pub struct UserInformation {
    pub name: String,
    pub birthday: String,
    pub mobile: String,
    pub e_mail: String,
    pub school: String,
}

#[derive(GraphQLObject)]
/// Basic object for every online-judge user.
pub struct User {
    pub token: String,
    /// Unique identification for each user.
    /// Can be used to login.
    pub username: String,
    /// Key used for login.
    /// Usually it can't be queried by the frontend.
    pub password: String,
    pub information: UserInformation,
    pub identity: UserIdentity,
    pub property: UserProperty,
}