use chrono::NaiveDate;
use graphql_client::*;
use seed::browser::service::fetch::Method::Post;
use seed::browser::service::fetch::ResponseDataResult;
use seed::{prelude::*, *};
use serde::Deserialize;

// Model
struct Model {
    pub customers: Option<CustomerFetchResult>,
}

impl Default for Model {
    fn default() -> Self {
        Self { customers: None }
    }
}

// GraphQL
#[derive(GraphQLQuery)]
#[graphql(
    query_path = "customers.graphql",
    schema_path = "schema.graphql",
    response_derives = "Debug,Clone"
)]
struct AllCustomers;
#[allow(non_camel_case_types)]
type date = NaiveDate;
#[warn(non_camel_case_types)]
#[derive(Clone, Deserialize, Debug)]
pub struct CustomerFetchResult {
    pub data: Option<all_customers::ResponseData>,
    pub errors: Option<Vec<graphql_client::Error>>,
}

// Update

#[derive(Clone)]
enum Msg {
    FetchCustomers,
    CustomerResultFetched(ResponseDataResult<CustomerFetchResult>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchCustomers => {
            model.customers = None;
            orders.perform_cmd(fetch_customers(all_customers::Variables {}));
        }
        Msg::CustomerResultFetched(Ok(response_data)) => {
            model.customers = Some(response_data);
        }
        Msg::CustomerResultFetched(Err(fail_reason)) => {
            model.customers = None;
            error!(fail_reason);
        }
    }
}

// View

fn customer_component(model: &Model) -> Node<Msg> {
    if let Some(customers) = &model.customers {
        let mut customer_rows: Vec<Node<Msg>> = vec![];
        if let Some(data) = &customers.data {
            for customer in &data.demo_customers {
                customer_rows.push(tr![
                    td![customer.id],
                    td![customer
                        .names
                        .clone()
                        .into_iter()
                        .map(|name| name.name)
                        .collect::<Vec<String>>()
                        .join(", ")],
                    td![customer
                        .identifiers
                        .clone()
                        .into_iter()
                        .map(|identifier| identifier.identifier)
                        .collect::<Vec<String>>()
                        .join(", ")],
                    td![customer
                        .classes_of_trade
                        .clone()
                        .into_iter()
                        .map(|class_of_trade| {
                            if let Some(class_of_trade) = class_of_trade.class_of_trade {
                                class_of_trade.name
                            } else {
                                "None".to_string()
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(", ")],
                    td![customer
                        .addresses
                        .clone()
                        .into_iter()
                        .map(|addresses| { addresses.address1 })
                        .collect::<Vec<String>>()
                        .join(", ")]
                ]);
            }
        }
        div![table![
            tr![
                th!["id"],
                th!["names"],
                th!["identifiers"],
                th!["classes of trade"],
                th!["addresses"]
            ],
            customer_rows
        ]]
    } else {
        div![]
    }
}

fn view(model: &Model) -> impl View<Msg> {
    div![
        h1!["GraphQL Rust Example"],
        customer_component(&model),
        div![
            attrs! {At::Class => "uk-grid"},
            div![button![
                simple_ev(Ev::Click, Msg::FetchCustomers),
                "Get customers".to_string()
            ]]
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}

async fn fetch_customers(variables: all_customers::Variables) -> Result<Msg, Msg> {
    let request_body = AllCustomers::build_query(variables);
    Request::new("https://hasura.yolo.dev.chronicledtest.net/v1/graphql".to_string())
        .method(Post)
        .body_json(&request_body)
        .fetch_json_data(Msg::CustomerResultFetched)
        .await
}
