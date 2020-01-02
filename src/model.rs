use seed::browser::service::fetch::ResponseDataResult;
use serde::Deserialize;

// Model
pub struct Model {
  pub customers: Option<AllCustomersFetchResult>,
  pub page: Page,
  pub customer: Option<CustomerFetchResult>,
}

#[derive(Debug)]
pub enum Page {
  Home,
  Customers,
  Customer,
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
  AllCustomersResultFetched(Box<ResponseDataResult<AllCustomersFetchResult>>),
  ChangeCustomerPage(String),
  CustomerResultFetched(Box<ResponseDataResult<CustomerFetchResult>>),
  CustomersPage,
  FetchCustomers,
  HomePage,
  RouteCustomerPage(String),
  RouteCustomersPage,
  RouteHomePage,
}

#[derive(Clone, Deserialize, Debug)]
pub struct AllCustomersFetchResult {
  pub data: Option<crate::graphql::customer::all_customers::ResponseData>,
  pub errors: Option<Vec<graphql_client::Error>>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct CustomerFetchResult {
  pub data: Option<crate::graphql::customer::customer::ResponseData>,
  pub errors: Option<Vec<graphql_client::Error>>,
}
