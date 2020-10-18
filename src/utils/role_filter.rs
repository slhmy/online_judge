pub fn customize_role(raw: &str) -> String {
    match raw {
        "student" => "student".to_owned(),
        "teacher" => "teacher".to_owned(),
        "admin" => "admin".to_owned(),
        _ => "net_friend".to_owned(),
    }
}