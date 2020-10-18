pub trait RegexMatcher {
    fn is_email(&self) -> bool;
    fn is_mobile(&self) -> bool;
    fn is_password(&self) -> bool;
}

use crate::*;
impl RegexMatcher for String{
    fn is_email(&self) -> bool { RE_EMAIL.is_match(&self) }
    fn is_mobile(&self) -> bool { RE_MOBILE.is_match(&self) }
    fn is_password(&self) -> bool { RE_PASSWORD.is_match(&self) }
}