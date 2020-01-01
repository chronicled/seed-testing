use crate::model::{Model, Msg};
use seed::{prelude::*, *};

pub fn home_view(model: &Model) -> Node<Msg> {
  div![
    attrs! {At::Class => "uk-grid"},
    div![button![
      simple_ev(Ev::Click, Msg::FetchCustomers),
      "Get customers".to_string()
    ]]
  ]
}
