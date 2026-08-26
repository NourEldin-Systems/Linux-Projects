use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Style {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Regular,
}

impl Style {
    // Returns (Markdown Prefix, Default Font Size)
    fn defaults(&self) -> (&'static str, &'static str) {
        match self {
            Style::H1 => ("# ", "text-4xl font-bold"),
            Style::H2 => ("## ", "text-3xl font-bold"),
            Style::H3 => ("### ", "text-2xl font-bold"),
            Style::H4 => ("#### ", "text-xl font-bold"),
            Style::H5 => ("##### ", "text-lg font-bold"),
            Style::H6 => ("###### ", "text-base font-bold"),
            Style::Regular => ("", "text-base"),
        }
    }
}

#[derive(Debug)]
enum TextSegment {
    Normal(String),
    Bold(String),
}

#[derive(Debug)]
struct Line {
    style: Style,
    font_size: String,
    segments: Vec<TextSegment>,
}

impl Line {
    // Factory function that parses raw text for **bold** syntax
    fn new(style: Style, raw_text: &str) -> Self {
        let (prefix, default_font) = style.defaults();
        let mut segments = Vec::new();

        // Simple markdown bold parser splitting by "**"
        let parts: Vec<&str> = raw_text.split("**").collect();
        let mut is_bold = false;

        for part in parts {
            if !part.is_empty() {
                if is_bold {
                    segments.push(TextSegment::Bold(part.to_string()));
                } else {
                    segments.push(TextSegment::Normal(part.to_string()));
                }
            }
            // Toggle bold state for the next segment
            is_bold = !is_bold;
        }

        Line {
            style,
            font_size: default_font.to_string(),
            segments,
        }
    }

    // Custom font size setter (override)
    fn with_font_size(mut self, size: &str) -> Self {
        self.font_size = size.to_string();
        self
    }

    // Prints out the final Markdown string
    fn to_md(&self) {
        let (prefix, _) = self.style.defaults();
        print!("{}", prefix);

        for segment in &self.segments {
            match segment {
                TextSegment::Normal(txt) => print!("{}", txt),
                TextSegment::Bold(txt) => print!("**{}**", txt),
            }
        }
        println!();
    }

    // Simulates an HTML/CSS render using the "font size" prop
    fn render_html(&self) {
        let tag = match self.style {
            Style::H1 => "h1",
            Style::H2 => "h2",
            Style::H3 => "h3",
            Style::H4 => "h4",
            Style::H5 => "h5",
            Style::H6 => "h6",
            Style::Regular => "p",
        };

        print!("<{} class=\"{}\">", tag, self.font_size);
        for segment in &self.segments {
            match segment {
                TextSegment::Normal(txt) => print!("{}", txt),
                TextSegment::Bold(txt) => print!("<strong>{}</strong>", txt),
            }
        }
        println!("</{}>", tag);
    }
}

// Helper factory functions for your note-taking
fn h1(text: &str) -> Line {
    Line::new(Style::H1, text)
}
fn h2(text: &str) -> Line {
    Line::new(Style::H2, text)
}
fn text(text: &str) -> Line {
    Line::new(Style::Regular, text)
}

fn main() {
    h1(
        "So I really have no idea about what I am currently doing and I don't even want to know what am I doing, but I have just made this file and it's compeletely useless and I don't care about it or what it does at all by any meaning, so I am just writing a buch of text here because I want to code and I should be coding right now but I feel too bored to actually just start simply coding, or should I? I don't even know what I am supposed to do right now in this current moment, maybe I shouldn't even wait for motivation to come becasue it will never propably and I should just start working and typing as fast as I am currently doing right now in this specific moment, ok? or not?",
    ).to_md();

    text("I mean this is really just some shits right now and I don't even know what am I typing right now, I just want to be typing fast but I don't even know what to type or what to build and this feels like real shit so I might as well open the editor and just start coding like an idiot, becuase I am propably an idiot, but I don't know, who knows, I know that I know nothing").to_md();
}
