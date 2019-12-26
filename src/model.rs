use serde::Deserialize;
use seed::browser::service::fetch::ResponseDataResult;

// Model
pub struct Model {
  pub customers: Option<CustomerFetchResult>,
}

impl Default for Model {
  fn default() -> Self {
      Self { customers: None }
  }
}

#[derive(Clone)]
pub enum Msg {
    FetchCustomers,
    CustomerResultFetched(Box<ResponseDataResult<CustomerFetchResult>>),
}

#[derive(Clone, Deserialize, Debug)]
pub struct CustomerFetchResult {
    pub data: Option<crate::graphql::customer::all_customers::ResponseData>,
    pub errors: Option<Vec<graphql_client::Error>>,
}
