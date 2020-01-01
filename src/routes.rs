use crate::model::Msg;
use seed::Url;

pub fn routes(url: Url) -> Option<Msg> {
  if url.path.is_empty() {
    return Some(Msg::CustomersPage);
  }

  Some(match url.path[0].as_ref() {
    "customer" => {
      // Determine if we're at the main guide page, or a subpage
      match url.path.get(1) {
        Some(id) => Msg::ChangeCustomerPage(id.to_string()),
        None => Msg::CustomersPage,
      }
    }
    _ => Msg::HomePage,
  })
}
