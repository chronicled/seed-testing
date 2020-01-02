use crate::model::{Model, Msg};
use seed::{prelude::*, *};

pub fn customer_view(model: &Model) -> Node<Msg> {
  if model.customer.is_none() {
    return div!["Loading customer"];
  }
  let customer = model.customer.clone().unwrap();
  if customer.errors.is_some() {
    let errors = customer.errors.unwrap();
    let string_errors = errors
      .iter()
      .map(|error| error.message.clone())
      .collect::<Vec<String>>()
      .join(" ");
    return div![string_errors];
  }
  if customer.data.is_none() {
    return div![];
  }
  let customer = customer.data.unwrap();
  if customer.demo_customers.len() > 1 {
    return div!["Unexpected multiple responses"];
  }
  if customer.demo_customers.is_empty() {
    return div!["No customers matched"];
  }
  let customer = customer.demo_customers.get(0).unwrap();
  div![p![customer.id]]
}
