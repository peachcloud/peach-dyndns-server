/*
*
* /register-user (sends an email verification to create a new account) NOT IMPLEMENTED
* /verify (for clicking the link in the email) NOT IMPLEMENTED
* /register-domain (add a new domain and get back the TSIG key for subsequent updating with nsupdate)
*/
use crate::generate_zone::{check_domain_available, generate_zone};
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[get("/")]
pub fn index() -> &'static str {
    "This is the peach-dyndns server."
}

#[derive(Serialize)]
pub struct JsonResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

// helper function to build a JsonResponse object
pub fn build_json_response(
    status: String,
    data: Option<JsonValue>,
    msg: Option<String>,
) -> JsonResponse {
    JsonResponse { status, data, msg }
}

#[derive(Deserialize, Debug)]
pub struct RegisterDomainPost {
    domain: String,
}

#[post("/register-domain", data = "<data>")]
pub async fn register_domain(data: Json<RegisterDomainPost>) -> Json<JsonResponse> {
    info!("++ post request to register new domain: {:?}", data);
    // TODO: first confirm domain is in the right format ("*.dyn.peachcloud.org")
    let is_domain_available = check_domain_available(&data.domain);
    if !is_domain_available{
        let status = "error".to_string();
        let msg = "can't register a domain that is already registered".to_string();
        Json(build_json_response(status, None, Some(msg)))
    } else {
        let result = generate_zone(&data.domain);
        match result {
            Ok(key_file_text) => {
                let status = "success".to_string();
                let msg = key_file_text.to_string();
                Json(build_json_response(status, None, Some(msg)))
            }
            Err(_err) => {
                let status = "error".to_string();
                let msg = "there was an error creating the zone file".to_string();
                Json(build_json_response(status, None, Some(msg)))
            }
        }
    }
}
