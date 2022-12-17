// Dave's Dashboard Browser based system and world dashboard
// Copyright (C) 2022  Dave Warnock dwarnock@gmail.com

// This file is part of Dave's Dashboard.

//Dave's Dashboard is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

// Dave's Dashboard is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License along with Dave's Dashboard.
// If not, see <https://www.gnu.org/licenses/>.

use crate::views::sys_info_view::sys_info_template;
use crate::models::sys_info_model::sys_info_as_json;

pub async fn get_sys_info() -> String {
    sys_info_template(sys_info_as_json())
}

pub async fn post_sys_info() -> String {
    "Hello World post from my own file".to_owned()
}