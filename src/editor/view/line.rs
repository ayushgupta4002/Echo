use core::fmt;
use std::ops::Range;
use unicode_width::UnicodeWidthStr;
use unicode_segmentation::UnicodeSegmentation;
#[derive(Copy, Clone , Debug)]
enum GraphemeWidth {
    Half,
    Full,
}

impl GraphemeWidth {
    const fn saturating_add(self, other: usize) -> usize {
        match self {
            Self::Half => other.saturating_add(1),
            Self::Full => other.saturating_add(2), 
        }
    }
}
#[derive(Debug)]
struct TextFragment {
    grapheme: String,
    rendered_width: GraphemeWidth,
    replacement: Option<char>,
}
#[derive( Debug , Default)]
pub struct Line {
    fragments: Vec<TextFragment>,
}
impl Line {
    pub fn from(line_str: &str) -> Self { 
        let fragments = Self::str_to_fragments(line_str);
        Self { fragments }
    }

    fn str_to_fragments(line_str : &str)-> Vec<TextFragment>{
        line_str
            .graphemes(true) 
            .map(|grapheme| {
                let (replacement, rendered_width) = Self::replace_char(grapheme).map_or_else(
                    || {
                        let unicode_width = grapheme.width();
                        let rendered_width = match unicode_width {
                            0 | 1 => GraphemeWidth::Half,
                            _ => GraphemeWidth::Full,
                        };
                        (None, rendered_width)
                    },
                    |replacement| (Some(replacement), GraphemeWidth::Half),
                );

                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                } 
            })
            .collect()

    }


    fn replace_char(str: &str) -> Option<char> {
        let width = str.width();
        match str {
            " " => None,
            "\t" => Some(' '),
            _ if width > 0 && str.trim().is_empty() => Some('␣'),
            _ => None,
        }
    }

    pub fn get_visible_graphemes(&self, range: Range<usize>) -> String {
        if range.start >= range.end {
            return String::new();
        } 
        let mut result = String::new();
        let mut current_pos = 0; 
        for fragment in &self.fragments {
            let fragment_end = fragment.rendered_width.saturating_add(current_pos); 
            if current_pos >= range.end {
                break;
            } 
            if fragment_end > range.start { 
                if fragment_end > range.end || current_pos < range.start {
                    // Clip on the right or left
                    result.push('⋯'); 
                } else if let Some(char) = fragment.replacement {
                    result.push(char); 
                } else {
                    result.push_str(&fragment.grapheme); 
                }
            }
            current_pos = fragment_end;
        }
        result
    }
    
    pub fn grapheme_count(&self) -> usize { 
        self.fragments.len()
    }

    pub fn width_until(&self, grapheme_index: usize) -> usize { 
        self.fragments
            .iter()
            .take(grapheme_index) 
            .map(|fragment| match fragment.rendered_width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2, 
            })
            .sum() 
    }


    pub fn insert_char(&mut self, character: char, grapheme_index: usize) {
        let mut result = String::new();

        for (index, fragment) in self.fragments.iter().enumerate() { 
            if index == grapheme_index { 
                result.push(character);
            }
            result.push_str(&fragment.grapheme); 
        }
        if grapheme_index >= self.fragments.len() {
            result.push(character);
        } 
        self.fragments = Self::str_to_fragments(&result); 
    }

    pub fn delete(&mut self, grapheme_index: usize) {
        if grapheme_index >= self.fragments.len() {
            return;
        }
        let mut result = String::new();
        for (index, fragment) in self.fragments.iter().enumerate() {
            if index != grapheme_index {
                result.push_str(&fragment.grapheme);
            }
        }
        self.fragments = Self::str_to_fragments(&result);
    }

    pub fn append(&mut self, other: &Self) {
        let mut concat = self.to_string();
        concat.push_str(&other.to_string());
        self.fragments = Self::str_to_fragments(&concat); 
    }

    pub fn split(&mut self, grapheme_index: usize) -> Self {
        if grapheme_index == self.fragments.len() {
            return Self::default();
        }
        let remainder = self.fragments.split_off(grapheme_index);
        Self { fragments: remainder }
    }
}


impl fmt::Display for Line {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let result: String = self
            .fragments
            .iter()
            .map(|fragment| fragment.grapheme.clone())
            .collect();
        write!(formatter, "{result}") 
    }
}