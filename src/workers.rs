pub mod customer;
use crate::model::{Model, Msg, Page};
use seed::prelude::*;

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
  match msg {
    Msg::FetchCustomers => {
      model.customers = None;
      model.page = Page::Loading;
      orders.perform_cmd(customer::fetch_all_customers());
    }
    Msg::AllCustomersResultFetched(response_data) => {
      customer::update_all_customers(response_data, model);
      model.page = Page::Customers;
    }
    Msg::HomePage => model.page = Page::Home,
    Msg::CustomersPage => model.page = Page::Customers,
    Msg::CustomerPage => {
      model.page = Page::Customer;
    }
    Msg::ChangeCustomerPage(customer_id) => {
      model.page = Page::Loading;
      orders.perform_cmd(customer::fetch_customer(customer_id));
    }
    Msg::CustomerResultFetched(response_data) => {
      customer::update_customer(response_data, model);
      model.page = Page::Customer;
    }
  }
}
