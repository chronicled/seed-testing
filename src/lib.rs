use seed::{*, prelude::*};
use seed::browser::service::fetch::ResponseDataResult;
use serde::{Deserialize, Serialize};

// Model

struct Model {
    pub ip: Option<IpFetchResult>,
    pub headers: Option<HeaderFetchResult>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            ip: None,
            headers: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct IpFetchResult {
    pub origin: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HeaderFetchResult {
    pub headers: HeaderDetails,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct HeaderDetails {
    pub accept: String,
    #[serde(rename = "Accept-Encoding")]
    pub accept_encoding: String,
    #[serde(rename = "Accept-Language")]
    pub accept_language: String,
    pub host: String,
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
}
// Update

#[derive(Clone)]
enum Msg {
    FetchIp,
    FetchHeaders,
    IpResultFetched(ResponseDataResult<IpFetchResult>),
    HeaderResultFetched(ResponseDataResult<HeaderFetchResult>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchIp => {
            model.ip = None;
            orders.perform_cmd(fetch_ip());
        }
        Msg::FetchHeaders => {
            model.headers = None;
            orders.perform_cmd(fetch_headers());
        }
        Msg::IpResultFetched(Ok(response_data)) => {
            model.ip = Some(response_data);
        }
        Msg::IpResultFetched(Err(fail_reason)) => {
            model.ip = None;
            error!(fail_reason);
        }
        Msg::HeaderResultFetched(Ok(response_data)) => {
            model.headers = Some(response_data);
        }
        Msg::HeaderResultFetched(Err(fail_reason)) => {
            model.headers = None;
            error!(fail_reason);
        }
    }
}

// View

fn ip_component(model: &Model) -> Node<Msg> {
    if model.ip.is_some() {
        div![format!(
            "Your IP Address is: {}",
            model.ip.as_ref().unwrap().origin
        )]
    } else {
        div![]
    }
}

fn header_component(model: &Model) -> Node<Msg> {
    if model.headers.is_some() {
        let headers = model.headers.as_ref().unwrap();

        div![ol![
            li![headers.headers.accept],
            li![headers.headers.accept_encoding],
            li![headers.headers.accept_language],
            li![headers.headers.host],
            li![headers.headers.user_agent],
        ]]
    } else {
        div![]
    }
}

fn view(model: &Model) -> impl View<Msg> {
    div![
        h1!["IP Detection"],
        ip_component(&model),
        header_component(&model),
        button![simple_ev(Ev::Click, Msg::FetchIp), "Detect IP".to_string()],
        button![
            simple_ev(Ev::Click, Msg::FetchHeaders),
            "Detect Headers".to_string()
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}

async fn fetch_ip() -> Result<Msg, Msg> {
    Request::new("https://httpbin.org/ip".to_string())
        .fetch_json_data(Msg::IpResultFetched)
        .await
}

async fn fetch_headers() -> Result<Msg, Msg> {
    Request::new("https://httpbin.org/headers".to_string())
        .fetch_json_data(Msg::HeaderResultFetched)
        .await
}
