use chrono::NaiveDate;
use graphql_client::*;

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

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "src/graphql/customer.graphql",
    schema_path = "src/graphql/schema.graphql",
    response_derives = "Debug,Clone"
)]
pub struct Customer;
