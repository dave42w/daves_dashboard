use axum::Json;

use sysinfo::{System, SystemExt};
// removed unused NetworkExt, ProcessExt, NetworksExt,

use serde::Serialize;

#[derive(Serialize)]
pub struct SysInfo {
    system_name: String,
    kernel_version: String,
    os_version: String,
    host_name: String
}

pub fn sys_info_as_json() -> Json<SysInfo> {
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
