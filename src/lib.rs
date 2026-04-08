#![allow(clippy::wildcard_imports)]

use worker::*;

#[event(fetch)]
async fn fetch(
    req: Request,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    let mut url = req.url()?;
    url.set_host(Some("emeditor.org"))?;
    let new_url = url;

    let mut new_req = Request::new(new_url.as_str(), req.method())?;
    
    // Copy headers from the original request
    let headers = req.headers();
    let new_headers = new_req.headers_mut()?;
    for (name, value) in headers {
        if name.to_lowercase() == "host" {
            continue;
        }
        new_headers.set(&name, &value)?;
    }
    new_headers.set("host", new_url.host_str().unwrap_or("emeditor.org"))?;

    console_log!("Request Headers: {:?}", new_req.headers());

    let resp = Fetch::Request(new_req).send().await?;

    console_log!("Response Status: {}", resp.status_code());
    console_log!("Response Headers: {:?}", resp.headers());

    Ok(resp)
}