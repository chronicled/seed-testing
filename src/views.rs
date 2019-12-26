mod customer;
use crate::model::{Model, Msg};
use seed::{prelude::*, *};

pub fn index_view(model: &Model) -> impl View<Msg> {
  div![
    h1!["GraphQL Rust Example"],
    customer::customer_component(&model),
    div![
      attrs! {At::Class => "uk-grid"},
      div![button![
        simple_ev(Ev::Click, Msg::FetchCustomers),
        "Get customers".to_string()
      ]]
    ]
  ]
}
