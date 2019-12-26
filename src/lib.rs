use seed::prelude::*;
mod graphql;
mod model;
mod views;
mod workers;
use views::index_view;

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(workers::update, index_view).build_and_start();
}
