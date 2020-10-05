use crate::graphql_schema::objects::{
    user::*,
};

pub fn get_user_by_token(token: &str) -> User {
    User {
        token: "MY_TOKEN".to_owned(),
        username: "slhmy".to_owned(),
        password: "1234".to_owned(),
        information: UserInformation {
            name: "Sam".to_owned(),
            birthday: "1998/11/09".to_owned(),
            mobile: "13585581170".to_owned(),
            e_mail: "1484836413@qq.com".to_owned(),
            school: "Shanghai University".to_owned(),
        },
        identity: UserIdentity::Student,
        property: UserProperty {
            status: Vec::new(),
        },
    }
}