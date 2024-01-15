
use reqwest::*;
use crate::utils::*;

fn chomp_httpo(idx: usize) -> std::result::Result<String, ()> {
    if let Some(urls) = get_option_from_metadata("urls") {
        if let Some(root_url) = urls.get(idx) {
            let root_url = root_url.to_string();
            return Ok(drag_er_under(&root_url))
        }
    }
    Err(())
}

fn drag_er_under(url: &str) -> String {
    let content = match blocking::get(url) {
        Ok(response) => response.text().unwrap_or_else(|_| croc_doc!{}),
        Err(_) => croc_doc!{},
    };
    todo![]
}







