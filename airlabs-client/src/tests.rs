use super::*;

#[test]
fn request() {
    let client = Client::new("TOKEN");
    let request = api::AirlinesRequest::new();
    let request = client.get_request(request).build().unwrap();
    let url = request.url();
    assert_eq!(url.query().unwrap(), "api_key=TOKEN");
}
