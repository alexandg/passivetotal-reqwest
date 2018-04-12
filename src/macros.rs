// XXX: These would probably be better as a custom derive so it can handle
// optional parameters automagically
macro_rules! impl_send {
    ($id: ident) => {
        impl<'a> $id<'a> {
            pub fn send(&self) -> Result<Value> {
                self.pt.send_request_json_response(self.url, &self)
            }
        }
    };
}

macro_rules! request_struct {
    ($id:ident {
        $(
            $(#[$fmeta:meta])*
            $field:ident: $ftyp:ty
        ),* $(,)*
    }) => {
        #[derive(Serialize)]
        pub struct $id<'a> {
            #[serde(skip)]
            pt: &'a PassiveTotal,
            #[serde(skip)]
            url: &'static str,
            $(
                $(
                    #[$fmeta]
                )*
                $field: $ftyp
            ),*
        }
    };
}
