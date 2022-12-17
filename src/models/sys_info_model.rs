// Dave's Dashboard Browser based system and world dashboard
// Copyright (C) 2022  Dave Warnock dwarnock@gmail.com

// This file is part of Dave's Dashboard.

//Dave's Dashboard is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

// Dave's Dashboard is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License along with Dave's Dashboard.
// If not, see <https://www.gnu.org/licenses/>.

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
