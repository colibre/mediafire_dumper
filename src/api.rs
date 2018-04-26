

mod api {
    extern crate serde_json;
    extern crate reqwest;
    type Value = serde_json::Value;
    type Client = reqwest::Client;
    
    macro_rules! gen_folder_api_call {
        ($call_name:ident) => (
            pub fn $call_name(client: Option<Client>, params: &[(String, String)]) -> Option<Value> {
                let reply = if let Some(client) = client {
                    let mut request = client.post("http://www.mediafire.com/api/1.5/folder/$call_name.php")
                                            .query(params)
                                            .send()
                                            .unwrap();
                    request.text()
                    } else {
                    let client = Client::new();
                    let mut request = client.post("http://www.mediafire.com/api/1.5/folder/$call_name.php")
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

        gen_folder_api_call!(get_content);
    }
}
