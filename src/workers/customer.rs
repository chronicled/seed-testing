use crate::graphql::customer::all_customers;
use crate::graphql::customer::customer;
use crate::graphql::customer::AllCustomers;
use crate::graphql::customer::Customer;
use crate::model::AllCustomersFetchResult;
use crate::model::CustomerFetchResult;
use crate::model::{Model, Msg};
use graphql_client::*;
use seed::browser::service::fetch::Method::Post;
use seed::browser::service::fetch::ResponseDataResult;
use seed::*;

pub async fn fetch_all_customers() -> Result<Msg, Msg> {
  let request_body = AllCustomers::build_query(all_customers::Variables {});
  Request::new("https://hasura.yolo.dev.chronicledtest.net/v1/graphql".to_string())
    .method(Post)
    .body_json(&request_body)
    .fetch_json_data(|response| Msg::AllCustomersResultFetched(Box::new(response)))
    .await
}

pub async fn fetch_customer(customer_id: String) -> Result<Msg, Msg> {
  let request_body = Customer::build_query(customer::Variables { customer_id });
  Request::new("https://hasura.yolo.dev.chronicledtest.net/v1/graphql".to_string())
    .method(Post)
    .body_json(&request_body)
    .fetch_json_data(|response| Msg::CustomerResultFetched(Box::new(response)))
    .await
}

pub fn update_all_customers(
  response_data: Box<ResponseDataResult<AllCustomersFetchResult>>,
  model: &mut Model,
) {
  let raw = Box::leak(response_data);
  if let Ok(customers) = raw {
    model.customers = Some(customers.clone());
  }
  if let Err(fail_reason) = raw {
    model.customers = None;
    error!(fail_reason);
  }
}

pub fn update_customer(
  response_data: Box<ResponseDataResult<CustomerFetchResult>>,
  model: &mut Model,
) {
  let raw = Box::leak(response_data);
  if let Ok(customer) = raw {
    model.customer = Some(customer.clone());
  }
  if let Err(fail_reason) = raw {
    model.customers = None;
    error!(fail_reason);
  }
}
