use worker::*;

#[event(fetch)]
async fn fetch(
    req: Request,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    let url = req.url()?;
    let new_url_str = url.as_str().replace("emeditor.org", "help.emeditor.com");
    let new_url = Url::parse(&new_url_str)?;

    let mut new_req = Request::new(new_url.as_str(), req.method())?;
    
    // Copy headers from the original request
    let headers = req.headers();
    let new_headers = new_req.headers_mut()?;
    for (name, value) in headers {
        new_headers.set(&name, &value)?;
    }

    Fetch::Request(new_req).send().await
}