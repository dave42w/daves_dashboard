use std::ops::Add;

use crate::views::sys_info_view::sys_info_template;
use crate::models::sys_info_model::sys_info_as_json;

// this should use the model to get the data as JSON, 
// then pass the json data to the view and return the view string
pub async fn get_sys_info() -> String {
    
    //test();
    //sys_info.to_owned()
    sys_info_as_json();
    let s: String = sys_info_template().to_string();
    let s = s.add(" CONTROLLER ");
    s.to_owned()
}

pub async fn post_sys_info() -> String {
    "Hello World post from my own file".to_owned()
}