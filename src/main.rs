use std::collections::HashMap;
use macro1_macros::{DeriveCustomModel, IntoHashMap};
use std::convert::From;

#[derive(DeriveCustomModel)]
#[custom_model(model(
    name = "UserName",
    fields(first_name, last_name),
    extra_derives(IntoHashMap)
))]
#[custom_model(model(name = "UserInfo", fields(username, age), extra_derives(Debug, Clone)))]
#[allow(unused)]
pub struct User {
    username: String,
    first_name: String,
    last_name: String,
    age: u32,
}



fn main() {
    let user_name = UserName {
        first_name: "Heiko".into(),
        last_name: "Vogeler".into(),
    };

    let hashm = HashMap::<String, String>::from(user_name);
    dbg!(hashm);

    let user_info = UserInfo {
        username: "hvogeler".into(),
        age: 62,
    };

    println!("{}-{}", user_info.username, user_info.age);
    dbg!(user_info);
}
