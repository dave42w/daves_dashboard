// template logic for sys_view
use axum::Json;
//use serde::Serialize;
use crate::models::sys_info_model::SysInfo;
//use axum::Json;
use handlebars::Handlebars;



// I want this to get the data as JSON, then render using a template into a String
pub fn sys_info_template(j: Json<SysInfo>) -> String {
    let mut hbr = Handlebars::new();
    let source = "system_name: {{system_name}}, kernel_version: {{kernel_version}}, os_version: {{os_version}}, host_name: {{host_name}}.".to_owned();
    
    let _ = hbr.register_template_string("t1", source);
    hbr.render("t1", &j.0).unwrap()
    
}
