use crate::model::{Model, Msg};
use seed::{prelude::*, *};

pub fn all_customers_view(model: &Model) -> Node<Msg> {
    if let Some(customers) = &model.customers {
        let mut customer_rows: Vec<Node<Msg>> = vec![];
        if let Some(data) = &customers.data {
            for customer in &data.demo_customers {
                customer_rows.push(tr![
                    td![a![
                        attrs! {At::Class => ".uk-text-primary"},
                        simple_ev(Ev::Click, Msg::RouteCustomerPage(customer.id.clone())),
                        customer.id
                    ]],
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
        div![attrs! {At::AttributeName => "uk-spinner"}]
    }
}
