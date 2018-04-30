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
#[derive(Debug, Deserialize)]
pub struct Data<T: Debug> {
    data: T,
}

/// When a query returns 'resources', these resources are transmitted only as
/// references. i.e. the type and ID of the resource is returned, allowing one
/// to make further queries accordingly or extract them from the 'included' object.
#[derive(Debug, Deserialize)]
pub struct ResourceId {
    #[serde(rename="type")]
    type_: String,
    // TODO It's really an int though.
    id: String,
}

#[derive(Debug, Deserialize)]
pub struct TypedResource<Attr: Debug, Rel: Debug> {
    // Because this is strongly typed, we already know the value of the 'type' field.
    // Let serde manage it whenever we deserialize into an enum (where type is one of many).
    //#[serde(rename="type")]
    //type_: String,
    // TODO It's really an int though.
    id: String,
    attributes: Attr,
    /// When accessed through the "included" field, no relationships are shown.
    relationships: Option<Rel>,
    // TODO: learn more about these types and make them type-safe
    #[serde(default)]
    links: HashMap<String, SerdeUrl>,
    #[serde(default)]
    meta: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
/// List of differently sized avatar images available
pub struct Avatar {
    // TODO: how many of these sizes are optional?
    // 192 was not found for https://www.fimfiction.net/api/v2/groups/209275
    // TODO: Maybe some sparse vector type is better.
    #[serde(with = "url_serde", rename="16")]
    size16: Url,
    #[serde(with = "url_serde", rename="32")]
    size32: Url,
    #[serde(with = "url_serde", rename="48")]
    size48: Url,
    #[serde(with = "url_serde", rename="64")]
    size64: Url,
    #[serde(with = "url_serde", rename="96")]
    size96: Url,
    #[serde(with = "url_serde", rename="128")]
    size128: Url,
    #[serde(with = "url_serde", rename="192")]
    size192: Option<Url>,
    #[serde(with = "url_serde", rename="256")]
    size256: Url,
    #[serde(with = "url_serde", rename="384")]
    size384: Url,
    #[serde(with = "url_serde", rename="512")]
    size512: Url,
}

/// 'color' struct, as serialized by Fimfiction's API,
/// e.g. the 'color' field within a story.
#[derive(Debug, Deserialize)]
pub struct Color {
    hex: String,
    rgb: [u8; 3],
}
/// Links to where the cover image for a story may be found.
/// One link per each size of the story.
#[derive(Debug, Deserialize)]
pub struct CoverImage {
    #[serde(with = "url_serde")]
    thumbnail: Url,
    #[serde(with = "url_serde")]
    medium: Url,
    #[serde(with = "url_serde")]
    large: Url,
    #[serde(with = "url_serde")]
    full: Url,
}
/// Bookshelf icon.
/// It appears the bookshelf icon is glyph from a font
#[derive(Debug, Deserialize)]
pub struct Icon {
    name: String,
    #[serde(rename="type")]
    type_: String,
    data: String,
}

/// Position of a author's note.
#[derive(Debug, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Position {
    Top,
    Bottom,
}
/// Privacy settings for a story
#[derive(Debug, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Privacy {
    Private,
    Unlisted,
    Public
}

/// Story publish status
#[derive(Debug, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum PublishStatus {
    Visible,
    NotVisible,
    ApproveQueue,
    PostQueue,
}
/// Story completion status
#[derive(Debug, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum CompletionStatus {
    Incomplete,
    Complete,
    Hiatus,
    Cancelled,
}
/// Story content rating
#[derive(Debug, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum ContentRating {
    Everyone,
    Teen,
    Mature,
}
/// Story tag type
#[derive(Debug, Deserialize)]
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




#[derive(Debug, Deserialize)]
pub struct BlogPostAttributes {
    /// Title of the blog post
    title: String,
    /// Date the blog entry was posted
    date_posted: DateTime<Utc>,
    /// HTML marked up truncated intro of the post
    intro: Option<String>,
    ///// Content of the blog post
    // TODO: object
    //content: Option<String>,
    /// HTML version of content
    content_html: Option<String>,
    /// Number of views the blog post has
    num_views: u32,
    /// Number of comments the blog post has
    num_comments: u32,
    /// Whether the post is a site post or not
    site_post: bool,
    /// The site post tag of this post. Only returned if site_post is true
    // TODO: Should this be `TagType'?
    site_post_tag: Option<String>,
    /// Array of tags on this blog post
    // TODO: Should this be `TagType'?
    tags: Vec<String>,
}
#[derive(Debug, Deserialize)]
pub struct BlogPostRelationships {
    author: Data<ResourceId>,
    tagged_story: Data<ResourceId>,
}
pub type BlogPost = TypedResource<BlogPostAttributes, BlogPostRelationships>;

#[derive(Debug, Deserialize)]
pub struct BookshelfAttributes {
    name: String,
    privacy: Privacy,
    description: String,
    // TODO: dedicated 'color' type?
    color: String,
    icon: Icon,
    num_stories: u32,
    num_unread: u32,
    track_unread: bool,
    quick_add: bool,
    email_on_update: bool,
    date_created: DateTime<Utc>,
    date_modified: DateTime<Utc>,
    order: u32,
}
#[derive(Debug, Deserialize)]
pub struct BookshelfRelationships {
    // TODO: 'user' relationship wasn't documented, but is present for /api/v2/bookshelves/16299
    user: Data<ResourceId>,
    // TODO: 'story' relationship was documented, but not present for /api/v2/bookshelves/16299
    //story: Data<ResourceId>,
}
pub type Bookshelf = TypedResource<BookshelfAttributes, BookshelfRelationships>;

#[derive(Debug, Deserialize)]
pub struct ChapterAttributes {
    chapter_number: u32,
    title: String,
    published: bool,
    num_views: u32,
    date_published: DateTime<Utc>,
    date_modified: DateTime<Utc>,
    content: Option<String>,
    content_html: Option<String>,
    // TODO:
    //authors_note: Option<object>
    authors_note_html: Option<String>,
    authors_note_position: Position,
}
#[derive(Debug, Deserialize)]
pub struct ChapterRelationships {
    story: Data<ResourceId>,
}
pub type Chapter = TypedResource<ChapterAttributes, ChapterRelationships>;

#[derive(Debug, Deserialize)]
pub struct FollowAttributes {
    date_followed: DateTime<Utc>,
}
#[derive(Debug, Deserialize)]
pub struct FollowRelationships {
    user: Data<ResourceId>,
    following: Data<ResourceId>,
}
pub type Follow = TypedResource<FollowAttributes, FollowRelationships>;


#[derive(Debug, Deserialize)]
pub struct GroupAttributes {
    name: String,
    description: String,
    description_html: String,
    num_members: u32,
    num_stories: u32,
    nsfw: bool,
    open: bool,
    hidden: bool,
    date_created: DateTime<Utc>,

    // Undocumented:
    icon: Avatar,
}
#[derive(Debug, Deserialize)]
pub struct GroupRelationships {
    founder: Data<ResourceId>,
}
pub type Group = TypedResource<GroupAttributes, GroupRelationships>;


#[derive(Debug, Deserialize)]
pub struct GroupThreadAttributes {
    title: String,
    num_posts: u32,
    date_created: DateTime<Utc>,
    date_last_posted: DateTime<Utc>,
    sticky: bool,
    locked: bool,
}
#[derive(Debug, Deserialize)]
pub struct GroupThreadRelationships {
    creator: Data<ResourceId>,
    group: Data<ResourceId>,
    last_poster: Data<ResourceId>,
}
pub type GroupThread = TypedResource<GroupThreadAttributes, GroupThreadRelationships>;


#[derive(Debug, Deserialize)]
pub struct PrivateMessageAttributes {
    subject: String,
    // TODO
    //content: object
    content_html: String,
    date_sent: DateTime<Utc>,
    read: bool,
}
#[derive(Debug, Deserialize)]
pub struct PrivateMessageRelationships {
    sender: Data<UserAttributes>,
    receiver: Data<UserAttributes>,
}
pub type PrivateMessage = TypedResource<PrivateMessageAttributes, PrivateMessageRelationships>;

/// Data fimfiction returns about any single story.
/// See https://www.fimfiction.net/developers/api/v2/docs/resources#story
#[derive(Debug, Deserialize)]
pub struct StoryAttributes {
    /// The title of the story
    title: String,
    /// The short description of the story 
    short_description: String,
    /// The description of the story 
    description: String,
    /// HTML version of description
    description_html: String,
    /// Whether the story is published or not. Effectively the same as checking if status = visible	
    published: bool,
    /// The publish status of the story
    status: PublishStatus,
    /// Whether the story has been submitted or not. Set to true to submit the story
    submitted: bool,
    /// Date the story was first published
    date_published: DateTime<Utc>,
    /// Date the story was last modified. Updated whenever any edit is made to the story
    date_modified: DateTime<Utc>,
    /// Date the story was last updated. Only updated when a chapter is added and only if the last bump timing was more than 12 hours ago.
    date_updated: DateTime<Utc>,
    /// Number of views the story has (max on one chapter) 
    num_views: u32,
    /// Total number of views the story has (across all chapters)
    total_num_views: u32,
    /// Number of words the story has
    num_words: u32,
    /// Number of comments the story has
    num_comments: u32,
    /// Primary color for the story (based off cover art) 
    color: Color,
    /// The cover image for the story
    cover_image: CoverImage,

    // undocumented attributes below
    num_chapters: u32,
    rating: u32,
    completion_status: CompletionStatus,
    content_rating: ContentRating,
    num_likes: u32,
    num_dislikes: u32,
}
#[derive(Debug, Deserialize)]
pub struct StoryRelationships {
    author: Data<ResourceId>,
    // TODO: this field isn't showing up on /stories/:id requests
    //chapters: Data<Vec<ResourceId>>,
    tags: Data<Vec<ResourceId>>,
    // TODO: this field isn't showing up on /stories/:id requests
    //prequel: Data<ResourceId>,
}
pub type Story = TypedResource<StoryAttributes, StoryRelationships>;


#[derive(Debug, Deserialize)]
pub struct StoryTagAttributes {
    name: String,
    description: Option<String>,
    #[serde(rename="type")]
    type_: TagType,
    num_stories: u32,
}
pub type StoryTag = TypedResource<StoryTagAttributes, ()>;


#[derive(Debug, Deserialize)]
pub struct UserAttributes {
    name: String,
    // fimfiction docs advertise an 'email' field, but it doesn't exist.
    //email: String,
    // TODO
    //bio: object
    bio_html: String,
    num_followers: u32,
    num_stories: u32,
    num_blog_posts: u32,
    date_joined: DateTime<Utc>,
    avatar: Avatar,

    // undocumented
    color: Color,
    // Doesn't seem to be present when accessed from a story's 'included' resources
    date_last_online: Option<DateTime<Utc>>,
}
pub type User = TypedResource<UserAttributes, ()>;


#[derive(Debug, Deserialize)]
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
