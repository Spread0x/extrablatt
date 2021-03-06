use std::ops::Deref;

use select::node::Node;
use select::predicate::{Attr, Name, Predicate};
use url::{ParseError, Url};

#[derive(Debug, Clone)]
pub enum VideoProvider {
    Youtube,
    Vimeo,
    Dailymotion,
    Other(String),
}

impl VideoProvider {
    fn from_host(s: &str) -> Result<Self, Self> {
        if s.contains("youtube") || s.contains("youtu.be") {
            return Ok(VideoProvider::Youtube);
        }
        if s.contains("vimeo") {
            return Ok(VideoProvider::Vimeo);
        }
        if s.contains("dailymotion") {
            return Ok(VideoProvider::Dailymotion);
        }
        Err(VideoProvider::Other(s.to_string()))
    }
}

pub struct VideoNode<'a> {
    inner: Node<'a>,
}

impl<'a> VideoNode<'a> {
    /// The common predicate to identify Video node
    pub fn node_predicate() -> impl Predicate {
        Name("iframe").or(Name("object").or(Name("video")))
    }

    pub fn new(inner: Node<'a>) -> Self {
        Self { inner }
    }

    pub fn get_width(&self) -> Option<&str> {
        self.inner.attr("width")
    }

    pub fn get_height(&self) -> Option<&str> {
        self.inner.attr("height")
    }

    pub fn get_src(&self) -> Option<&str> {
        if Some("object") == self.inner.name() {
            self.inner
                .find(Name("param").and(Attr("name", "movie")))
                .filter_map(|n| n.attr("value"))
                .next()
        } else {
            self.inner.attr("src")
        }
    }

    pub fn get_src_url(&self, base_url: Option<&Url>) -> Option<Result<Url, ParseError>> {
        if let Some(url) = self.get_src() {
            let options = Url::options().base_url(base_url);
            Some(options.parse(url))
        } else {
            None
        }
    }

    pub fn get_provider(&self, base_url: Option<&Url>) -> Option<VideoProvider> {
        if let Some(url) = self.get_src_url(base_url) {
            if let Ok(url) = url {
                if let Some(host) = url.host_str() {
                    let provider = VideoProvider::from_host(host);
                    return match provider {
                        Ok(p) => Some(p),
                        Err(p) => Some(p),
                    };
                }
            }
        }
        None
    }
}

impl<'a> Deref for VideoNode<'a> {
    type Target = Node<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
