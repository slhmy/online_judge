pub fn customize_role(raw: &str) -> String {
    match raw {
        "student" => "student".to_owned(),
        "teacher" => "teacher".to_owned(),
        "admin" => "admin".to_owned(),
        "net_friend" => "net_friend".to_owned(),
        _ => "unlogged".to_owned(),
    }
}

pub fn role_level(raw: &str) -> i32 {
    match raw {
        "admin" => 4,
        "teacher" => 3,
        "student" => 2,
        "net_friend" => 0,
        _ => -1,
    }
}