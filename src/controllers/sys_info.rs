use std::ops::Add;

use axum::Json;
use serde::Serialize;

use sysinfo::{System, SystemExt};
// removed unused NetworkExt, ProcessExt, NetworksExt,

use crate::views::sys_info_view::sys_info_template;

#[derive(Serialize)]
pub struct SysInfo {
    system_name: String,
    kernel_version: String,
    os_version: String,
    host_name: String
}

pub fn sys_info() -> Json<SysInfo> {
    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!

    let mut sys = System::new_all();
    // First we update all information of our `System` struct.
    sys.refresh_all();

    // system information to JSON
    let sysinfo = SysInfo {
        system_name: sys.name().unwrap(),
        kernel_version: sys.kernel_version().unwrap(),
        os_version: sys.os_version().unwrap(),
        host_name: sys.host_name().unwrap()
    
    };
    Json(sysinfo)
}
// this should use the model to get the data as JSON, 
// then pass the json data to the view and return the view string
pub async fn get_sys_info() -> String {
    
    //test();
    //sys_info.to_owned()
    sys_info();
    let s: String = sys_info_template().to_string();
    let s = s.add(" CONTROLLER ");
    s.to_owned()
}

pub async fn post_sys_info() -> String {
    "Hello World post from my own file".to_owned()
}