use crate::{
    ast::{ExternalLink, HashTag, InternalLink, Page, Text},
    Bracket, BracketKind, Heading, LineKind, ListKind, Syntax, SyntaxKind,
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
    fn visit_bracket_emphasis(&mut self, emphasis: &crate::Emphasis) -> Option<TransformCommand> {
        let h_level = (self.h1_level + 1).saturating_sub(emphasis.bold);
        if h_level > 0
            && h_level <= self.h1_level
            && (self.bold_to_h || (!self.bold_to_h && emphasis.bold > 1))
        {
            Some(TransformCommand::Replace(Syntax::new(SyntaxKind::Bracket(
                Bracket::new(BracketKind::Heading(Heading::new(&emphasis.text, h_level))),
            ))))
        } else {
            None
        }
    }
}

pub struct MarkdownGenConfig {
    indent: String,
}

impl Default for MarkdownGenConfig {
    fn default() -> Self {
        Self {
            indent: "   ".to_string(),
        }
    }
}

pub struct MarkdownGen {
    document: String,
    config: MarkdownGenConfig,
}

impl MarkdownGen {
    pub fn new(config: MarkdownGenConfig) -> Self {
        Self {
            document: String::new(),
            config,
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
            if let LineKind::List(list) = &line.kind {
                let indent = self.config.indent.repeat(list.level - 1);
                match &list.kind {
                    ListKind::Disc => self.document.push_str(&format!("{}* ", indent)),
                    ListKind::Decimal => self.document.push_str(&format!("{}1. ", indent)),
                    _ => {}
                }
            }
            self.visit_line(line);
            self.document.push_str("\n");
        }
    }

    fn visit_hashtag(&mut self, hashtag: &HashTag) -> Option<TransformCommand> {
        self.document
            .push_str(&format!("[#{t}](#{t}.md)", t = hashtag.value));
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

    fn visit_bracket_emphasis(&mut self, emphasis: &crate::Emphasis) -> Option<TransformCommand> {
        let mut tmp = emphasis.text.clone();
        if emphasis.bold > 0 {
            tmp = format!("**{}**", tmp);
        }
        if emphasis.italic > 0 {
            tmp = format!("*{}*", tmp);
        }
        if emphasis.strikethrough > 0 {
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
