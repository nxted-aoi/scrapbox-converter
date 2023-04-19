use std::collections::HashMap;

use crate::{
    ast::{
        Bracket, BracketKind, Emphasis, ExternalLink, HashTag, Heading, InternalLink, Line, Page,
        Syntax, SyntaxKind, Text,
    },
    BlockQuote,
};

pub mod markdown;

#[derive(Debug)]
pub enum TransformCommand {
    Replace(Syntax),
    Delete,
}

pub trait Visitor {
    fn visit(&mut self, value: &mut Page) {
        self.visit_page(value);
    }

    fn visit_page(&mut self, value: &mut Page) {
        for line in value.lines.iter_mut() {
            self.visit_line(line);
        }
    }

    fn visit_line(&mut self, value: &mut Line) {
        let mut commands = HashMap::new();
        for (i, item) in value.values.iter().enumerate() {
            let command = self.visit_syntax(item);
            if let Some(c) = command {
                commands.insert(i, c);
            }
        }

        // Replace
        for (&i, command) in &commands {
            if let TransformCommand::Replace(s) = command {
                value.values[i] = s.clone();
            }
        }

        // Delete
        let mut i = 0;
        value.values.retain(|_| {
            let retain = if let Some(TransformCommand::Delete) = commands.get(&i) {
                false
            } else {
                true
            };
            i += 1;
            retain
        });
    }

    fn visit_syntax(&mut self, value: &Syntax) -> Option<TransformCommand> {
        match &value.kind {
            SyntaxKind::HashTag(v) => self.visit_hashtag(&v),
            SyntaxKind::Bracket(v) => self.visit_bracket(&v),
            SyntaxKind::BlockQuote(v) => self.visit_block_quote(&v),
            SyntaxKind::Text(v) => self.visit_text(&v),
        }
    }

    fn visit_hashtag(&mut self, _value: &HashTag) -> Option<TransformCommand> {
        None
    }

    fn visit_bracket(&mut self, value: &Bracket) -> Option<TransformCommand> {
        match &value.kind {
            BracketKind::InternalLink(v) => self.visit_bracket_internal_link(&v),
            BracketKind::ExternalLink(v) => self.visit_bracket_external_link(&v),
            BracketKind::Emphasis(v) => self.visit_bracket_emphasis(&v),
            BracketKind::Heading(v) => self.visit_bracket_heading(&v),
        }
    }

    fn visit_bracket_internal_link(&mut self, _value: &InternalLink) -> Option<TransformCommand> {
        None
    }

    fn visit_bracket_external_link(&mut self, _value: &ExternalLink) -> Option<TransformCommand> {
        None
    }

    fn visit_bracket_emphasis(&mut self, _value: &Emphasis) -> Option<TransformCommand> {
        None
    }

    fn visit_bracket_heading(&mut self, _value: &Heading) -> Option<TransformCommand> {
        None
    }

    fn visit_block_quote(&mut self, _value: &BlockQuote) -> Option<TransformCommand> {
        None
    }

    fn visit_text(&mut self, _value: &Text) -> Option<TransformCommand> {
        None
    }
}
