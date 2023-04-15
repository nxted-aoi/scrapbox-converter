use crate::ast::{ExternalLink, HashTag, InternalLink, Page, Text};

use super::Visitor;

pub struct MarkdownGen {
    document: String,
}

impl MarkdownGen {
    pub fn new() -> Self {
        Self {
            document: String::new(),
        }
    }

    pub fn generate(&mut self, page: &Page) -> String {
        self.visit_page(page);
        self.document.clone()
    }
}

impl Visitor for MarkdownGen {
    fn visit_page(&mut self, page: &Page) {
        for line in &page.lines {
            self.visit_line(line);
            self.document.push_str("\n");
        }
    }

    fn visit_hashtag(&mut self, hashtag: &HashTag) {
        self.document.push_str(&format!("#{}", hashtag.value))
    }

    fn visit_bracket_internal_link(&mut self, link: &InternalLink) {
        self.document
            .push_str(&format!("[{t}]({t}).md", t = link.title));
    }

    fn visit_bracket_external_link(&mut self, link: &ExternalLink) {
        if let Some(title) = &link.title {
            self.document
                .push_str(&format!("[{}]({})", title, link.url));
        } else {
            self.document.push_str(&format!("{}", link.url));
        }
    }

    fn visit_text(&mut self, text: &Text) {
        self.document.push_str(&format!("{}", text.value));
    }
}


