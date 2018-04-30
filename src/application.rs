use reqwest;
use reqwest::{Client, header, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use url_serde;

use resources::{Resource, StoryAttributes};

/// Applications allow for the server to associate each request with some context
/// (i.e. some application). https://www.fimfiction.net/developers/api/v2/docs/applications
#[derive(Debug)]
pub struct Application {
    client: Client,
    /// Header used to authorize any requests with fimfiction.
    /// TODO: Can use Bearer?
    auth_header: header::Authorization<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    data: Resource,
    //included: Vec<Resource>,
    //#[serde(with="url_serde")]
    //uri: Url,
    //method: String,
    // TODO
    //debug: object
}



impl Application {
    /// Authorize an application via client-specific credentials.
    /// See https://www.fimfiction.net/developers/api/v2/docs/oauth#authorisation-code
    pub fn authorize_from_client_credentials(client_id: &str, client_secret: &str) -> Result<Self, reqwest::Error> {
        let client = Client::new();
        let post_data: [(&str, &str); 3] = [
            ("client_id", &client_id),
            ("client_secret", &client_secret),
            ("grant_type", "client_credentials"),
        ];
        #[derive(Debug, Deserialize)]
        struct RespData {
            access_token: String,
            token_type: String,
        }
        let mut response = client.post(Self::endpoint("token"))
            .header(Self::ua_header())
            .form(&post_data)
            .send()?;
        println!("{:?}", response);
        let resp_data: RespData = response.json()?;
        println!("authorized: {:?}", resp_data);
        Ok(Self{
            client,
            auth_header: header::Authorization(resp_data.token_type + " " + &resp_data.access_token),
        })
    }
    /// Retrieve a story by its id
    pub fn story(&self, story_id: u32) -> Result<ApiResponse, reqwest::Error> {
        self.do_request(
            self.client.get(Self::endpoint(format!("stories/{}", story_id)))
        )
    }
    /// Build the full URL to the given endpoint
    fn endpoint<T: AsRef<str>>(tail: T) -> Url {
        Url::parse("https://www.fimfiction.net/api/v2/").unwrap().join(tail.as_ref()).unwrap()
    }
    fn do_request<T: DeserializeOwned>(&self, mut req: RequestBuilder) -> Result<T, reqwest::Error> {
        let mut resp = req.header(self.auth_header.clone())
            .header(Self::ua_header())
            .send()?;
        println!("resp: {:?}", resp);
        //println!("resp.text: {:?}", resp.text());
        resp.json()
    }
    /// User-agent 
    fn ua_header() -> header::UserAgent {
        header::UserAgent::new("rust-fimfiction-api")
    }
}
