use reqwest::blocking::*;

pub fn get_content(link: String) -> String
{
    let client = Client::new();
    let request_builder = client.get(&link);
    let result = request_builder.send();
    return match result
    {
        Ok(x) => {
            let content = x.text();
            content.unwrap()
        },
        Err(x) => {
            println!("Can't access to: {}", &link);
            dbg!(&x);
            String::new()
        },
    }
}
