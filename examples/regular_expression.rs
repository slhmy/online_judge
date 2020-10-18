#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;

lazy_static! {
    static ref RE_EMAIL: Regex = Regex::new(r"^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$").unwrap();
    static ref RE_MOBILE: Regex = Regex::new(r"^((13[0-9])|(14[5|7])|(15([0-3]|[5-9]))|(18[0,5-9]))\d{8}$").unwrap();
    static ref RE_PASSWORD: Regex = Regex::new(r"^\S{6,20}$").unwrap();
}

trait RegexMatcher {
    fn is_email(&self) -> bool;
    fn is_mobile(&self) -> bool;
    fn is_password(&self) -> bool;
}

impl RegexMatcher for String{
    fn is_email(&self) -> bool { RE_EMAIL.is_match(&self) }
    fn is_mobile(&self) -> bool { RE_MOBILE.is_match(&self) }
    fn is_password(&self) -> bool { RE_PASSWORD.is_match(&self) }
}

fn main() {
    println!("{}", String::from("username@email.com").is_email());
    println!("{}", String::from("13585581170").is_mobile());
    println!("{}", String::from("123456").is_password());
}