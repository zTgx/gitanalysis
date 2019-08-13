pub mod basic;
pub mod oauth2;

pub enum AuthenType {
    Basic(String),
    Oauth2(String),
}
