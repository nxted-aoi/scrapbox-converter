#[derive(Debug, Clone, PartialEq, Default)]
pub struct Page {
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub kind: LineKind,
    pub values: Vec<Syntax>,
}

impl Line {
    pub fn new(kind: LineKind, values: Vec<Syntax>) -> Self {
        Self { kind, values }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineKind {
    Normal,
    List(List),
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub kind: ListKind,
    pub level: usize,
}

impl List {
    pub fn new(kind: ListKind, level: usize) -> Self {
        Self { kind, level }
    }

    pub fn disc(level: usize) -> Self {
        Self {
            kind: ListKind::Disc,
            level,
        }
    }

    pub fn decimal(level: usize) -> Self {
        Self {
            kind: ListKind::Decimal,
            level,
        }
    }

    pub fn alphabet(level: usize) -> Self {
        Self {
            kind: ListKind::Alphabet,
            level,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListKind {
    Disc,
    Decimal,
    Alphabet,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Syntax {
    pub kind: SyntaxKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxKind {
    HashTag(HashTag),
    Bracket(Bracket),
    BlockQuote(BlockQuote),
    Text(Text),
}

impl Syntax {
    pub fn new(kind: SyntaxKind) -> Self {
        Self { kind }
    }
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

impl Bracket {
    pub fn new(kind: BracketKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BracketKind {
    InternalLink(InternalLink),
    ExternalLink(ExternalLink),
    Emphasis(Emphasis),
    Heading(Heading),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockQuote {
    pub value: String,
}

impl BlockQuote {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
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

#[derive(Debug, Clone, PartialEq, Default)]
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

#[derive(Debug, Clone, PartialEq, Default)]
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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Heading {
    pub text: String,
    pub level: u8,
}

impl Heading {
    pub fn new(text: &str, level: u8) -> Self {
        Self {
            text: text.to_string(),
            level,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Emphasis {
    pub text: String,
    pub bold: u8,
    pub italic: u8,
    pub strikethrough: u8,
}

impl Emphasis {
    pub fn new(text: &str, bold: u8, italic: u8, strikethrough: u8) -> Self {
        Self {
            text: text.to_string(),
            bold,
            italic,
            strikethrough,
        }
    }

    pub fn bold(text: &str) -> Self {
        Self {
            text: text.to_string(),
            bold: 1,
            ..Default::default()
        }
    }

    pub fn bold_level(text: &str, level: u8) -> Self {
        Self {
            text: text.to_string(),
            bold: level,
            ..Default::default()
        }
    }

    pub fn italic(text: &str) -> Self {
        Self {
            text: text.to_string(),
            italic: 1,
            ..Default::default()
        }
    }

    pub fn strikethrough(text: &str) -> Self {
        Self {
            text: text.to_string(),
            strikethrough: 1,
            ..Default::default()
        }
    }
}
