extern crate serde_json;
extern crate reqwest;

macro_rules! gen_api_call {
    ($category:ident, $endpoint:ident) => (
        pub fn $endpoint(client: Option<&Client>, params: &[(&str, &str)]) -> Option<Value> {
            let reply = if let Some(client) = client {
                let mut request = client.post(concat!("http://www.mediafire.com/api/1.5/",
                                                      stringify!($category),
                                                      "/",
                                                      stringify!($endpoint),
                                                      ".php")
                                             )
                                        .query(params)
                                        .send()
                                        .unwrap();
                request.text()
            } else {
                let client = Client::new();
                let mut request = client.post(concat!("http://www.mediafire.com/api/1.5/",
                                                      stringify!($category),
                                                      "/",
                                                      stringify!($endpoint),
                                                      ".php")
                                             )
                                        .query(params)
                                        .send()
                                        .unwrap();
                request.text()
            };
            let v: Result<Value, _> = ::serde_json::from_str(&reply.unwrap_or_default());
            if v.is_ok() {
                Some(v.unwrap())
            } else {
                None
            }
        }
    )
}

pub mod folder {
    type Value = super::serde_json::Value;
    type Client = super::reqwest::Client;
    gen_api_call!(folder, get_content);
    gen_api_call!(folder, get_info);
}
