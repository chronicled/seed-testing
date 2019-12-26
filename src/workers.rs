pub mod customer;
use crate::graphql::customer::all_customers;
use crate::model::{Model, Msg};
use seed::{prelude::*};

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
  match msg {
    Msg::FetchCustomers => {
      model.customers = None;
      orders.perform_cmd(customer::fetch_customers(all_customers::Variables {}));
    }
    Msg::CustomerResultFetched(response_data) => {
      customer::update_model(response_data, model);
    }
  }
}
