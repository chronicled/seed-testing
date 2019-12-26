use crate::graphql::customer::all_customers::Variables;
use crate::graphql::customer::AllCustomers;
use crate::model::{CustomerFetchResult, Model, Msg};
use graphql_client::*;
use seed::browser::service::fetch::Method::Post;
use seed::browser::service::fetch::ResponseDataResult;
use seed::*;

pub async fn fetch_customers(variables: Variables) -> Result<Msg, Msg> {
  let request_body = AllCustomers::build_query(variables);
  Request::new("https://hasura.yolo.dev.chronicledtest.net/v1/graphql".to_string())
    .method(Post)
    .body_json(&request_body)
    .fetch_json_data(|response| Msg::CustomerResultFetched(Box::new(response)))
    .await
}

pub fn update_model(
  response_data: Box<ResponseDataResult<CustomerFetchResult>>,
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
