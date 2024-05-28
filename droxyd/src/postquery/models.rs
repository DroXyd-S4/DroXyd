use rocket::{FromForm};
#[derive(FromForm, Debug)]
pub struct SearchRequest {
    #[field(validate=len(1..))]
    pub(crate) request: String,
    #[field(validate=len(0..))]
    pub(crate) results_number: String,
}

impl SearchRequest
{
    pub fn get_request(&mut self) -> String
    {
        String::from(&self.request)
    }
    pub fn get_results(&mut self) -> String
    {
        String::from(&self.results_number)
    }
}
