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
use crate::models::sys_info_model::SysInfo;
use handlebars::Handlebars;

// TODO now use a template file
pub fn sys_info_template(j: Json<SysInfo>) -> String {
    let mut hbr = Handlebars::new();
    let source = "system_name: {{system_name}}, kernel_version: {{kernel_version}}, os_version: {{os_version}}, host_name: {{host_name}}.".to_owned();
    
    // TODO use result properly
    let _ = hbr.register_template_string("t1", source);
    hbr.render("t1", &j.0).unwrap()
}
