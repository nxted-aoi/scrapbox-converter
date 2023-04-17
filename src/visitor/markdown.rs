use crate::{
    ast::{ExternalLink, HashTag, InternalLink, Page, Text},
    Bracket, BracketKind, Heading, Syntax, SyntaxKind,
};

use super::{TransformCommand, Visitor};

pub struct MarkdownPass {
    pub h1_level: u8,
    pub bold_to_h: bool,
}

impl Default for MarkdownPass {
    fn default() -> Self {
        Self {
            h1_level: 3,
            bold_to_h: false,
        }
    }
}

impl Visitor for MarkdownPass {
    fn visit_bracket_decoration(
        &mut self,
        decoration: &crate::Decoration,
    ) -> Option<TransformCommand> {
        let h_level = (self.h1_level + 1).saturating_sub(decoration.bold);
        if h_level > 0
            && h_level <= self.h1_level
            && (self.bold_to_h || (!self.bold_to_h && decoration.bold > 1))
        {
            Some(TransformCommand::Replace(Syntax::new(SyntaxKind::Bracket(
                Bracket::new(BracketKind::Heading(Heading::new(
                    &decoration.text,
                    h_level,
                ))),
            ))))
        } else {
            None
        }
    }
}

pub struct MarkdownGen {
    document: String,
}

impl MarkdownGen {
    pub fn new() -> Self {
        Self {
            document: String::new(),
        }
    }

    pub fn generate(&mut self, page: &mut Page) -> String {
        self.visit(page);
        self.document.clone()
    }
}

impl Visitor for MarkdownGen {
    fn visit_page(&mut self, page: &mut Page) {
        for line in page.lines.iter_mut() {
            self.visit_line(line);
            self.document.push_str("\n");
        }
    }

    fn visit_hashtag(&mut self, hashtag: &HashTag) -> Option<TransformCommand> {
        self.document.push_str(&format!("[#{t}](#{t}.md)", t = hashtag.value));
        None
    }

    fn visit_bracket_internal_link(&mut self, link: &InternalLink) -> Option<TransformCommand> {
        self.document
            .push_str(&format!("[{t}]({t}).md", t = link.title));
        None
    }

    fn visit_bracket_external_link(&mut self, link: &ExternalLink) -> Option<TransformCommand> {
        if let Some(title) = &link.title {
            self.document
                .push_str(&format!("[{}]({})", title, link.url));
        } else {
            self.document.push_str(&format!("{}", link.url));
        }
        None
    }

    fn visit_text(&mut self, text: &Text) -> Option<TransformCommand> {
        self.document.push_str(&format!("{}", text.value));
        None
    }

    fn visit_bracket_decoration(
        &mut self,
        decoration: &crate::Decoration,
    ) -> Option<TransformCommand> {
        let mut tmp = decoration.text.clone();
        if decoration.bold > 0 {
            tmp = format!("**{}**", tmp);
        }
        if decoration.italic > 0 {
            tmp = format!("*{}*", tmp);
        }
        if decoration.strikethrough > 0 {
            tmp = format!("~~{}~~", tmp);
        }
        self.document.push_str(&tmp);
        None
    }

    fn visit_bracket_heading(&mut self, heading: &Heading) -> Option<TransformCommand> {
        self.document.push_str(&format!(
            "{} {}",
            "#".repeat(heading.level as usize),
            heading.text
        ));
        None
    }
}
