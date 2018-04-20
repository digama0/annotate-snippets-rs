use snippet::Snippet;
use std::fmt;

pub struct DisplayList {
    pub body: Vec<DisplayLine>,
}

impl From<Snippet> for DisplayList {
    fn from(snippet: Snippet) -> Self {
        let mut body = vec![];

        let mut current_line = snippet.slice.line_start;
        let mut current_index = 0;
        let mut line_index_ranges = vec![];

        for line in snippet.slice.source.lines() {
            body.push(DisplayLine::SourceLine {
                lineno: current_line,
                inline_marks: vec![],
                content: line.to_string(),
            });
            let line_length = line.chars().count() + 1;
            line_index_ranges.push((current_index, current_index + line_length));
            current_line += 1;
            current_index += line_length + 1;
        }
        // println!("{:?}", line_index_ranges);

        // range, label, id, annotation_type
        for annotation in snippet.annotations {}
        DisplayList { body }
    }
}

pub enum DisplayLine {
    RawLine(String),
    SourceLine {
        lineno: usize,
        inline_marks: Vec<DisplayMark>,
        content: String,
    },
    AnnotationLine {
        inline_marks: Vec<DisplayMark>,
        range: (usize, usize),
        label: String,
    },
    FoldLine,
}

#[derive(Debug)]
pub enum DisplayMark {
    AnnotationThrough,
    AnnotationStart,
}

impl fmt::Display for DisplayMark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DisplayMark::AnnotationThrough => write!(f, "|"),
            DisplayMark::AnnotationStart => write!(f, "/"),
        }
    }
}
