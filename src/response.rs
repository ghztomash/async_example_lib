pub struct Response {
    pub content: String,
}

impl Response {
    /// Create a new lookup response.
    pub fn new(content: String) -> Self {
        Response { content }
    }
}
