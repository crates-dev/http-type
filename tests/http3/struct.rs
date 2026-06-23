use crate::*;

pub(crate) async fn http3_test_handler(
    _request: Request,
    mut response: Response,
) -> Response {
    response.set_status_code(200).set_body("h3 ok");
    response
}
