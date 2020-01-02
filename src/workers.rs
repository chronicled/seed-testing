pub mod customer;
use crate::model::{Model, Msg, Page};
use seed::prelude::*;

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
  match msg {
    Msg::FetchCustomers => {
      model.customers = None;
      orders.skip().perform_cmd(customer::fetch_all_customers());
    }
    Msg::AllCustomersResultFetched(response_data) => {
      customer::update_all_customers(response_data, model);
    }
    Msg::HomePage => model.page = Page::Home,
    Msg::CustomersPage => model.page = Page::Customers,
    Msg::ChangeCustomerPage(customer_id) => {
      model.customer = None;
      model.page = Page::Customer;
      orders
        .skip()
        .perform_cmd(customer::fetch_customer(customer_id));
    }
    Msg::CustomerResultFetched(response_data) => {
      customer::update_customer(response_data, model);
    }
    Msg::RouteCustomerPage(customer_id) => {
      seed::push_route(vec![format!("/customer/{}", customer_id)]);
      orders.skip().send_msg(Msg::ChangeCustomerPage(customer_id));
    }
    Msg::RouteCustomersPage => {
      seed::push_route(vec!["/customers".to_string()]);
      orders.skip().send_msg(Msg::FetchCustomers);
      orders.skip().send_msg(Msg::CustomersPage);
    }
    Msg::RouteHomePage => {
      seed::push_route(vec!["/".to_string()]);
      orders.skip().send_msg(Msg::HomePage);
    }
  }
}
