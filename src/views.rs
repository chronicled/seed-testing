mod all_customers;
mod customer;
mod home;
use crate::model::{Model, Msg, Page};
use seed::{prelude::*, *};

pub fn index_view(model: &Model) -> impl View<Msg> {
  div![
    a![
      simple_ev(Ev::Click, Msg::RouteHomePage),
      "GraphQL Rust Example"
    ],
    match model.page {
      Page::Customers => all_customers::all_customers_view(&model),
      Page::Customer => customer::customer_view(&model),
      _ => home::home_view(&model),
    },
    div![
      attrs! {At::Class => "uk-card"},
      format!("Current page: {:?}", model.page)
    ]
  ]
}
