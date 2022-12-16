// Dave's Dashboard Browser based system and world dashboard
// Copyright (C) 2022  Dave Warnock dwarnock@gmail.com

// This file is part of Dave's Dashboard.

//Dave's Dashboard is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

// Dave's Dashboard is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License along with Dave's Dashboard.
// If not, see <https://www.gnu.org/licenses/>.

//use daves_dashboard::run;

mod routes;
mod controllers;
mod views;
mod models;

#[tokio::main]
async fn main() {
    run().await
}

pub async fn run() {
    let app = routes::create_routes();

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}