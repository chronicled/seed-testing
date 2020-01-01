use seed::browser::service::fetch::ResponseDataResult;
use serde::Deserialize;

// Model
pub struct Model {
  pub customers: Option<AllCustomersFetchResult>,
  pub page: Page,
  pub customer: Option<CustomerFetchResult>,
}

pub enum Page {
  Home,
  Customers,
  Customer,
  Loading,
}

impl Default for Model {
  fn default() -> Self {
    Self {
      customers: None,
      customer: None,
      page: Page::Home,
    }
  }
}

#[derive(Clone)]
pub enum Msg {
  FetchCustomers,
  AllCustomersResultFetched(Box<ResponseDataResult<AllCustomersFetchResult>>),
  CustomerResultFetched(Box<ResponseDataResult<CustomerFetchResult>>),
  CustomersPage,
  HomePage,
  CustomerPage,
  ChangeCustomerPage(String),
}

#[derive(Clone, Deserialize, Debug)]
pub struct AllCustomersFetchResult {
  pub data: Option<crate::graphql::customer::all_customers::ResponseData>,
  pub errors: Option<Vec<graphql_client::Error>>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct CustomerFetchResult {
  pub data: Option<crate::graphql::customer::all_customers::ResponseData>,
  pub errors: Option<Vec<graphql_client::Error>>,
}
