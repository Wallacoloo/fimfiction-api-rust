//! Contains all the structs defining "resources" with the fimfiction api may return.
use chrono::{DateTime, Utc};
use reqwest::Url;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;
use url_serde;
use url_serde::SerdeUrl;

/// Fimfiction often returns data inside a "data" key.
/// This object provides a way to replicate that wrapping on the rust side.
#[derive(Debug, Deserialize, Serialize)]
pub struct Data<T: Debug> {
    pub data: T,
}

/// When a query returns 'resources', these resources are transmitted only as
/// references. i.e. the type and ID of the resource is returned, allowing one
/// to make further queries accordingly or extract them from the 'included' object.
#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceId {
    #[serde(rename="type")]
    pub type_: String,
    // TODO It's really an int though.
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TypedResource<Attr: Debug, Rel: Debug> {
    // Because this is strongly typed, we already know the value of the 'type' field.
    // Let serde manage it whenever we deserialize into an enum (where type is one of many).
    //#[serde(rename="type")]
    //type_: String,
    // TODO It's really an int though.
    pub id: String,
    pub attributes: Attr,
    /// When accessed through the "included" field, no relationships are shown.
    pub relationships: Option<Rel>,
    // TODO: learn more about these types and make them type-safe
    #[serde(default)]
    pub links: HashMap<String, SerdeUrl>,
    #[serde(default)]
    pub meta: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
/// List of differently sized avatar images available
pub struct Avatar {
    // TODO: how many of these sizes are optional?
    // 192 was not found for https://www.fimfiction.net/api/v2/groups/209275
    // 16 was not found for id 33084
    // TODO: Maybe some sparse vector type is better.
    #[serde(default, with = "url_serde", rename="16")]
    pub size16: Option<Url>,
    #[serde(with = "url_serde", rename="32")]
    pub size32: Url,
    #[serde(with = "url_serde", rename="48")]
    pub size48: Url,
    #[serde(with = "url_serde", rename="64")]
    pub size64: Url,
    #[serde(with = "url_serde", rename="96")]
    pub size96: Url,
    #[serde(with = "url_serde", rename="128")]
    pub size128: Url,
    #[serde(default, with = "url_serde", rename="192")]
    pub size192: Option<Url>,
    #[serde(with = "url_serde", rename="256")]
    pub size256: Url,
    #[serde(with = "url_serde", rename="384")]
    pub size384: Url,
    #[serde(with = "url_serde", rename="512")]
    pub size512: Url,
}

/// 'color' struct, as serialized by Fimfiction's API,
/// e.g. the 'color' field within a story.
#[derive(Debug, Deserialize, Serialize)]
pub struct Color {
    pub hex: String,
    pub rgb: [u8; 3],
}
/// Links to where the cover image for a story may be found.
/// One link per each size of the story.
#[derive(Debug, Deserialize, Serialize)]
pub struct CoverImage {
    #[serde(with = "url_serde")]
    pub thumbnail: Url,
    #[serde(with = "url_serde")]
    pub medium: Url,
    #[serde(with = "url_serde")]
    pub large: Url,
    #[serde(with = "url_serde")]
    pub full: Url,
}
/// Bookshelf icon.
/// It appears the bookshelf icon is glyph from a font
#[derive(Debug, Deserialize, Serialize)]
pub struct Icon {
    pub name: String,
    #[serde(rename="type")]
    pub type_: String,
    pub data: String,
}

/// Position of a author's note.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="snake_case")]
pub enum Position {
    Top,
    Bottom,
}
/// Privacy settings for a story
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="snake_case")]
pub enum Privacy {
    Private,
    Unlisted,
    Public
}

/// Story publish status
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="snake_case")]
pub enum PublishStatus {
    Visible,
    NotVisible,
    ApproveQueue,
    PostQueue,
}
/// Story completion status
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="snake_case")]
pub enum CompletionStatus {
    Incomplete,
    Complete,
    Hiatus,
    Cancelled,
}
/// Story content rating
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="snake_case")]
pub enum ContentRating {
    Everyone,
    Teen,
    Mature,
}
/// Story tag type
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="snake_case")]
pub enum TagType {
    Character,
    Genre,
    Rating,
    Content,
    Series,
    Warning,
    Universe,
}




#[derive(Debug, Deserialize, Serialize)]
pub struct BlogPostAttributes {
    /// Title of the blog post
    pub title: String,
    /// Date the blog entry was posted
    pub date_posted: DateTime<Utc>,
    /// HTML marked up truncated intro of the post
    pub intro: Option<String>,
    ///// Content of the blog post
    // TODO: object
    //content: Option<String>,
    /// HTML version of content
    pub content_html: Option<String>,
    /// Number of views the blog post has
    pub num_views: u32,
    /// Number of comments the blog post has
    pub num_comments: u32,
    /// Whether the post is a site post or not
    pub site_post: bool,
    /// The site post tag of this post. Only returned if site_post is true
    // TODO: Should this be `TagType'?
    pub site_post_tag: Option<String>,
    /// Array of tags on this blog post
    // TODO: Should this be `TagType'?
    pub tags: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct BlogPostRelationships {
    pub author: Data<ResourceId>,
    pub tagged_story: Data<ResourceId>,
}
pub type BlogPost = TypedResource<BlogPostAttributes, BlogPostRelationships>;

#[derive(Debug, Deserialize, Serialize)]
pub struct BookshelfAttributes {
    pub name: String,
    pub privacy: Privacy,
    pub description: String,
    // TODO: dedicated 'color' type?
    pub color: String,
    pub icon: Icon,
    pub num_stories: u32,
    pub num_unread: u32,
    pub track_unread: bool,
    pub quick_add: bool,
    pub email_on_update: bool,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub order: u32,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct BookshelfRelationships {
    // TODO: 'user' relationship wasn't documented, but is present for /api/v2/bookshelves/16299
    pub user: Data<ResourceId>,
    // TODO: 'story' relationship was documented, but not present for /api/v2/bookshelves/16299
    //pub story: Data<ResourceId>,
}
pub type Bookshelf = TypedResource<BookshelfAttributes, BookshelfRelationships>;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChapterAttributes {
    pub chapter_number: u32,
    pub title: String,
    pub published: bool,
    pub num_views: u32,
    pub date_published: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub content: Option<String>,
    pub content_html: Option<String>,
    // TODO:
    //pub authors_note: Option<object>
    pub authors_note_html: Option<String>,
    pub authors_note_position: Position,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ChapterRelationships {
    pub story: Data<ResourceId>,
}
pub type Chapter = TypedResource<ChapterAttributes, ChapterRelationships>;

#[derive(Debug, Deserialize, Serialize)]
pub struct FollowAttributes {
    pub date_followed: DateTime<Utc>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct FollowRelationships {
    pub user: Data<ResourceId>,
    pub following: Data<ResourceId>,
}
pub type Follow = TypedResource<FollowAttributes, FollowRelationships>;


#[derive(Debug, Deserialize, Serialize)]
pub struct GroupAttributes {
    pub name: String,
    pub description: String,
    pub description_html: String,
    pub num_members: u32,
    pub num_stories: u32,
    pub nsfw: bool,
    pub open: bool,
    pub hidden: bool,
    pub date_created: DateTime<Utc>,

    // Undocumented:
    pub icon: Avatar,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct GroupRelationships {
    pub founder: Data<ResourceId>,
}
pub type Group = TypedResource<GroupAttributes, GroupRelationships>;


#[derive(Debug, Deserialize, Serialize)]
pub struct GroupThreadAttributes {
    pub title: String,
    pub num_posts: u32,
    pub date_created: DateTime<Utc>,
    pub date_last_posted: DateTime<Utc>,
    pub sticky: bool,
    pub locked: bool,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct GroupThreadRelationships {
    pub creator: Data<ResourceId>,
    pub group: Data<ResourceId>,
    pub last_poster: Data<ResourceId>,
}
pub type GroupThread = TypedResource<GroupThreadAttributes, GroupThreadRelationships>;


#[derive(Debug, Deserialize, Serialize)]
pub struct PrivateMessageAttributes {
    pub subject: String,
    // TODO
    //content: object
    pub content_html: String,
    pub date_sent: DateTime<Utc>,
    pub read: bool,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PrivateMessageRelationships {
    pub sender: Data<UserAttributes>,
    pub receiver: Data<UserAttributes>,
}
pub type PrivateMessage = TypedResource<PrivateMessageAttributes, PrivateMessageRelationships>;

/// Data fimfiction returns about any single story.
/// See https://www.fimfiction.net/developers/api/v2/docs/resources#story
#[derive(Debug, Deserialize, Serialize)]
pub struct StoryAttributes {
    /// The title of the story
    pub title: String,
    /// The short description of the story 
    pub short_description: String,
    /// The description of the story 
    pub description: String,
    /// HTML version of description
    pub description_html: String,
    /// Whether the story is published or not. Effectively the same as checking if status = visible	
    pub published: bool,
    /// The publish status of the story
    pub status: PublishStatus,
    /// Whether the story has been submitted or not. Set to true to submit the story
    pub submitted: bool,
    /// Date the story was first published
    pub date_published: DateTime<Utc>,
    /// Date the story was last modified. Updated whenever any edit is made to the story
    pub date_modified: DateTime<Utc>,
    /// Date the story was last updated. Only updated when a chapter is added and only if the last bump timing was more than 12 hours ago.
    pub date_updated: DateTime<Utc>,
    /// Number of views the story has (max on one chapter) 
    pub num_views: u32,
    /// Total number of views the story has (across all chapters)
    pub total_num_views: u32,
    /// Number of words the story has
    pub num_words: u32,
    /// Number of comments the story has
    pub num_comments: u32,
    /// Primary color for the story (based off cover art) 
    pub color: Color,
    /// The cover image for the story
    pub cover_image: CoverImage,

    // undocumented attributes below
    pub num_chapters: u32,
    pub rating: u32,
    pub completion_status: CompletionStatus,
    pub content_rating: ContentRating,
    pub num_likes: u32,
    pub num_dislikes: u32,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct StoryRelationships {
    pub author: Data<ResourceId>,
    // TODO: this field isn't showing up on /stories/:id requests
    //pub chapters: Data<Vec<ResourceId>>,
    pub tags: Data<Vec<ResourceId>>,
    // TODO: this field isn't showing up on /stories/:id requests
    //pub prequel: Data<ResourceId>,
}
pub type Story = TypedResource<StoryAttributes, StoryRelationships>;


#[derive(Debug, Deserialize, Serialize)]
pub struct StoryTagAttributes {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename="type")]
    pub type_: TagType,
    pub num_stories: u32,
}
pub type StoryTag = TypedResource<StoryTagAttributes, ()>;


#[derive(Debug, Deserialize, Serialize)]
pub struct UserAttributes {
    pub name: String,
    // fimfiction docs advertise an 'email' field, but it doesn't exist.
    //email: String,
    // TODO
    //pub bio: object
    pub bio_html: String,
    pub num_followers: u32,
    pub num_stories: u32,
    pub num_blog_posts: u32,
    pub date_joined: DateTime<Utc>,
    pub avatar: Avatar,

    // undocumented
    pub color: Color,
    // Doesn't seem to be present when accessed from a story's 'included' resources
    pub date_last_online: Option<DateTime<Utc>>,
}
pub type User = TypedResource<UserAttributes, ()>;


#[derive(Debug, Deserialize, Serialize)]
#[serde(tag="type", rename_all="snake_case")]
pub enum Resource {
    BlogPost(BlogPost),
    Bookshelf(Bookshelf),
    Chapter(Chapter),
    Follow(Follow),
    Group(Group),
    GroupThread(GroupThread),
    PrivateMessage(PrivateMessage),
    Story(Story),
    StoryTag(StoryTag),
    User(User),
}
