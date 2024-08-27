use crate::{
    body::Body,
    headers::{ContentType, Headers},
    response::Response,
    status_line::{Status, StatusLine},
};

#[derive(Clone, Debug, Default)]
pub struct ResponseBuilder {
    // http_version: Option<HttpVersion>,
    status: Option<Status>,
    headers: Headers,
    body: Option<Body>,
}

impl ResponseBuilder {
    pub fn set_status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }

    pub fn set_body(mut self, body: impl Into<Body>) -> Self {
        let body = body.into();
        let headers = self
            .headers
            .set_content_type(ContentType::Text)
            .set_content_length(body.len());
        self.headers = headers;
        self.body = Some(body);
        self
    }

    pub fn build(self) -> Response {
        let status_line = match self.status {
            None => StatusLine::default(),
            Some(status) => StatusLine::make_http_1_1_status_line(status),
        };

        Response::new(status_line, self.headers, self.body)
    }
}
