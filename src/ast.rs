#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub lines: Vec<Syntax>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Syntax {
    pub kind: SyntaxKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxKind {
    HashTag(HashTag),
    Bracket(Bracket),
    Text(Text),
}

#[derive(Debug, Clone, PartialEq)]
pub struct HashTag {
    pub value: String,
}

impl HashTag {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bracket {
    pub kind: BracketKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BracketKind {
    InternalLink(InternalLink),
    ExternalLink(ExternalLink),
    Decoration(Decoration),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub value: String,
}

impl Text {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InternalLink {
    pub title: String,
}

impl InternalLink {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExternalLink {
    pub title: Option<String>,
    pub url: String,
}

impl ExternalLink {
    pub fn new(title: Option<&str>, url: &str) -> Self {
        Self {
            title: title.map(String::from),
            url: url.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Decoration {
    pub text: String,
}

impl Decoration {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}
