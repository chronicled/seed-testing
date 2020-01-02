use crate::model::{Model, Msg};
use seed::{prelude::*, *};

pub fn home_view(_model: &Model) -> Node<Msg> {
  div![button![
    attrs! {At::Class => "uk-button uk-button-default"},
    simple_ev(Ev::Click, Msg::RouteCustomersPage),
    "Get customers".to_string()
  ]]
}
