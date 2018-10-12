use reqwest;
use reqwest::{Client, header, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;

use resources::{BlogPost, Bookshelf, Chapter, Group, PrivateMessage, Resource, Story, User};

/// Applications allow for the server to associate each request with some context
/// (i.e. some application). https://www.fimfiction.net/developers/api/v2/docs/applications
#[derive(Debug)]
pub struct Application {
    client: Client,
    /// Header used to authorize any requests with fimfiction.
    /// TODO: Can use header::Bearer?
    auth_header: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub data: Resource,
    pub included: Vec<Resource>,
    // NB: Url relative to fimfiction.net; cannot use Url type for that.
    pub uri: String,
    pub method: String,
    pub debug: HashMap<String, Value>,
}
#[derive(Debug, Deserialize)]
pub struct TypedApiResponse<T> {
    pub data: T,
    pub included: Vec<Resource>,
    // NB: Url relative to fimfiction.net; cannot use Url type for that.
    pub uri: String,
    pub method: String,
    pub debug: HashMap<String, Value>,
}
pub type BlogPostResponse = TypedApiResponse<BlogPost>;
pub type BookshelfResponse = TypedApiResponse<Bookshelf>;
pub type ChapterResponse = TypedApiResponse<Chapter>;
//pub type FollowersResponse = TypedApiResponse<Vec<Follow>>;
pub type GroupResponse = TypedApiResponse<Group>;
pub type PrivateMessageResponse = TypedApiResponse<PrivateMessage>;
pub type StoryResponse = TypedApiResponse<Story>;
pub type UserResponse = TypedApiResponse<User>;



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
            .header(header::USER_AGENT, Self::user_agent())
            .form(&post_data)
            .send()?;
        debug!("authorization response: {:?}", response);
        let resp_data: RespData = response.json()?;
        debug!("authorized: {:?}", resp_data);
        Ok(Self{
            client,
            auth_header: resp_data.token_type + " " + &resp_data.access_token,
        })
    }
    /// Retrieve a blogpost by its id (/blog-posts/:id).
    pub fn blog_post(&self, id: u32) -> Result<BlogPostResponse, reqwest::Error> {
        self.do_request(
            self.client.get(Self::endpoint(format!("blog-posts/{}", id)))
        )
    }
    /// Retrieve a bookshelf by its id (/blog-posts/:id).
    pub fn bookshelf(&self, id: u32) -> Result<BookshelfResponse, reqwest::Error> {
        self.do_request(
            self.client.get(Self::endpoint(format!("bookshelves/{}", id)))
        )
    }
    /// Retrieve a chapter by its id (/blog-posts/:id).
    pub fn chapter(&self, id: u32) -> Result<ChapterResponse, reqwest::Error> {
        self.do_request(
            self.client.get(Self::endpoint(format!("chapters/{}", id)))
        )
    }
    /// Retrieve a group by its id (/groups/:id).
    pub fn group(&self, id: u32) -> Result<GroupResponse, reqwest::Error> {
        self.do_request(
            self.client.get(Self::endpoint(format!("groups/{}", id)))
        )
    }
    /// Retrieve a private message by its id (/private-messages/:id).
    // NB: untested! Requires the read_pms scope.
    pub fn private_message(&self, id: u32) -> Result<PrivateMessageResponse, reqwest::Error> {
        self.do_request(
            self.client.get(Self::endpoint(format!("private-messages/{}", id)))
        )
    }
    /// Retrieve a story by its id (/stories/:id).
    pub fn story(&self, id: u32) -> Result<StoryResponse, reqwest::Error> {
        self.do_request(
            self.client.get(Self::endpoint(format!("stories/{}", id)))
        )
    }
    /// Retrieve a user by its id (/users/:id).
    pub fn user(&self, id: u32) -> Result<UserResponse, reqwest::Error> {
        self.do_request(
            self.client.get(Self::endpoint(format!("users/{}", id)))
        )
    }

    /// Build the full URL to the given endpoint
    fn endpoint<T: AsRef<str>>(tail: T) -> Url {
        Url::parse("https://www.fimfiction.net/api/v2/").unwrap().join(tail.as_ref()).unwrap()
    }
    fn do_request<T: DeserializeOwned>(&self, req: RequestBuilder) -> Result<T, reqwest::Error> {
        let mut resp = req.header(header::AUTHORIZATION, self.auth_header.clone())
            .header(header::USER_AGENT, Self::user_agent())
            .send()?;
        debug!("do_request response: {:?}", resp);
        //println!("resp.text: {}", resp.text().unwrap());
        resp.json()
    }
    fn user_agent() -> &'static str {
        "rust-fimfiction-api"
    }
}
