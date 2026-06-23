use crate::*;

pub(crate) async fn http2_test_handler(
    _request: Request,
    mut response: Response,
) -> Response {
    response.set_status_code(200).set_body("h2 ok");
    response
}
