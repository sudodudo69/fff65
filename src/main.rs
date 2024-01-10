use fastly::{Error, Request, Response};

#[fastly::main]
fn main(_req: Request) -> Result<Response, Error> {
    // Send two asynchronous backend requests, and store the pending requests in a vector
    // This endpoint returns "Cat" in the response body
    let req1 = Request::get("https://httpbin.org/base64/Q2F0")
        .send_async("origin_0")?;
    // This endpoint returns "Dog" in the response body
    let req2 = Request::get("https://httpbin.org/base64/RG9n")
        .send_async("origin_1")?;
    let pending_reqs = vec![req1, req2];

    // Wait for one of the requests to finish
    let (resp, pending_reqs) = fastly::http::request::select(pending_reqs);
    // Return an error if the request was not successful
    let mut resp1 = resp?;
    let (resp, _pending_reqs) = fastly::http::request::select(pending_reqs);
    let mut resp2 = resp?;

    // Create a new response from the two origin responses
    let resp = Response::from_body(format!(
        "{} responded first, {} responded next",
        resp1.take_body_str(),
        resp2.take_body_str()
    ));

    Ok(resp)
}