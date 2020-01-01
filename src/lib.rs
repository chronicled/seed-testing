use seed::prelude::*;
mod graphql;
mod model;
mod routes;
mod views;
mod workers;
use routes::routes;
use views::index_view;

#[wasm_bindgen(start)]
pub fn render() {
  App::builder(workers::update, index_view)
    .routes(routes)
    .build_and_start();
}
