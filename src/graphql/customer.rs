use graphql_client::*;
use chrono::NaiveDate;

// GraphQL
#[derive(GraphQLQuery)]
#[graphql(
    query_path = "src/graphql/customers.graphql",
    schema_path = "src/graphql/schema.graphql",
    response_derives = "Debug,Clone"
)]
pub struct AllCustomers;
#[allow(non_camel_case_types)]
type date = NaiveDate;
