#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

#[derive(Debug)]
pub enum HttpVersion {
    V1_1,
    V2_0,
    V3_0,
}
