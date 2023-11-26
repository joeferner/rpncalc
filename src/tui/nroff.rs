// see https://minix1.woodhull.com/current/2.0.4/wwwman/man7/man.7.html

const BOLD_START: &str = "\x1b[1m";
const BOLD_END: &str = "\x1b[0m";

pub fn nroff_format(text: &str, width: usize) -> String {
    let width = width.max(20);
    let mut results = "".to_string();
    let mut indent = 0;
    for line in text.split('\n') {
        if line.starts_with(".SH") {
            for part in format_subhead(line) {
                results.push_str(part.as_str());
            }
        } else if line.starts_with(".IP") {
            for part in format_indent_paragraph(line) {
                results.push_str(part.as_str());
            }
            indent = 5;
        } else if line.starts_with(".RE") {
            indent = 0.max(indent as i32 - 5) as usize;
        } else if line.is_empty() {
            results.push('\n');
        } else {
            for part in split_line(line, (width - indent).max(20)) {
                results.push_str(format!("{}{}", " ".repeat(indent), part).as_str());
            }
        }
    }
    return results;
}

fn format_subhead(line: &str) -> Vec<String> {
    if !line.starts_with(".SH") {
        panic!();
    }
    let mut line = line.get(".SH".len()..).unwrap().trim();
    if line.starts_with('\"') && line.ends_with('\"') {
        line = line.get(1..line.len() - 1).unwrap();
    }
    let result = format!("{}{}{}\n", BOLD_START, line, BOLD_END);
    return vec![result];
}

fn format_indent_paragraph(line: &str) -> Vec<String> {
    if !line.starts_with(".IP") {
        panic!();
    }
    let mut line = line.get(".IP".len()..).unwrap().trim();
    if line.starts_with('\"') && line.ends_with('\"') {
        line = line.get(1..line.len() - 1).unwrap();
    }
    let result = format!("{}\n", line);
    return vec![result];
}

fn split_line(line: &str, width: usize) -> Vec<String> {
    let mut line = line.to_string();
    let mut results: Vec<String> = Vec::new();
    while line.len() > width {
        let first_part = line.get(0..width).unwrap();
        let last_part = line.get(width..).unwrap();

        if let Some(space_parts) = first_part.rsplit_once(' ') {
            results.push(format!("{}\n", space_parts.0));
            line = format!("{}{}", space_parts.1, last_part);
        } else {
            results.push(format!("{}\n", first_part));
            line = last_part.to_string();
        }
    }
    if !line.is_empty() {
        results.push(format!("{}\n", line));
    }
    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_no_split() {
        assert_eq!(vec!["0123456789\n"], split_line("0123456789", 10));
    }

    #[test]
    fn test_split_line_last_space() {
        assert_eq!(vec!["0123 0123\n", "0123\n"], split_line("0123 0123 0123", 10));
    }

    #[test]
    fn test_split_line_no_good_split() {
        assert_eq!(vec!["0123456789\n", "0\n"], split_line("01234567890", 10));
    }

    #[test]
    fn test_split_line_multiple_splits() {
        assert_eq!(
            vec!["0123456789\n", "0\n", "012345678\n", "012345678\n"],
            split_line("01234567890 012345678 012345678", 10)
        );
    }
}
