// XXX: These would probably be better as a custom derive so it can handle
// optional parameters automagically
macro_rules! impl_send {
    ($id:ident) => {
        impl<'a> $id<'a> {
            pub fn send(&self) -> Result<Value> {
                self.pt.send_request_json_response(self.url, json!({}))
            }
        }
    };
}

macro_rules! impl_send_query {
    ($id:ident) => {
        impl<'a> $id<'a> {
            pub fn send(&self) -> Result<Value> {
                self.pt.send_request_json_response(
                    self.url,
                    json!({
                        "query": ::utils::valid_domain(&self.query)?
                    }),
                )
            }
        }
    };
}
