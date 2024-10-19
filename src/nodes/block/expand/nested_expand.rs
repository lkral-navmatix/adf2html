use serde::{Deserialize, Serialize};

use crate::ToHtml;
use super::{
    expand::expand_html_formatting, 
    super::{
        heading::Heading, 
        media::{media_group::MediaGroup, media_single::MediaSingle},
        paragraph::Paragraph, 
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NestedExpand {
    pub content: Vec<Content>,
    #[serde(rename = "attrs")]
    pub attributes: Attributes,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub title: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Content {
    Heading(Heading),
    MediaGroup(MediaGroup),
    MediaSingle(MediaSingle),
    Paragraph(Paragraph),
}

impl ToHtml for NestedExpand {
    fn to_html(&self) -> String {
        let title = self.attributes.title.clone().unwrap_or_default();
        let content = self.content
            .iter()
            .map(|n| n.to_html())
            .collect::<String>();

        expand_html_formatting(title, content)
    }
}

impl ToHtml for Content {
    fn to_html(&self) -> String {
        match self {
            Content::Heading(heading) => heading.to_html(),
            Content::MediaGroup(media_group) => media_group.to_html(),
            Content::MediaSingle(media_single) => media_single.to_html(),
            Content::Paragraph(paragraph) => paragraph.to_html(),
        }
    }
}

impl NestedExpand {
    pub(crate) fn replace_media_urls(&mut self, urls: &mut Vec<String>) {
        for content in self.content.iter_mut() {
            content.replace_media_urls(urls);
        }        
    }
}

impl Content {
    pub(crate) fn replace_media_urls(&mut self, urls: &mut Vec<String>) {
        match self {
            Content::Heading(_heading) => (),
            Content::MediaGroup(media_group) => media_group.replace_media_urls(urls),
            Content::MediaSingle(media_single) => media_single.replace_media_urls(urls),
            Content::Paragraph(_paragraph) => (),
        }
    }
}
