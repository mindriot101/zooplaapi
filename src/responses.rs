#[derive(Deserialize, Debug)]
pub(crate) struct SessionResponse {
    pub session_id: String,
    /*
    area_name: Option<String>,
    country: Option<String>,
    postcode: Option<String>,
    street: Option<String>,
    town: Option<String>,
    */
}
