//! What is listed of the matched files after the repeated lines and the limits (OUTPUT.md#limits-and-truncation).

use super::blocks::{BlockOptions, before, blocks};
use super::lines::{cut_line, lines_of};
use super::repeated::Repeated;
use super::{Fragment, Highlight, Limits, display_found};
use crate::content::Content;
use crate::search::Found;

/// What is listed of one file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Listed {
    /// No lines: the file has no fragments, or the detail lists no lines.
    NoLines,
    /// All its fragments are on repeated lines: the file is not listed.
    LeftOut,
    /// The fragments whose lines are listed, none when a limit cut them all.
    Lines(Vec<Fragment>),
}

/// What is listed of the listed files, and the limits which cut it, in the order of the limits table.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Listing {
    pub files: Vec<Listed>,
    /// Per file whether a per file limit cut it.
    pub cut: Vec<bool>,
    /// Per file the first line which is not listed, when `max_output_per_file` or `max_output` cut it.
    pub until: Vec<Option<usize>>,
    /// How many of the listed files are written: `max_output` leaves out the next ones.
    pub shown: usize,
    pub truncated_by: Vec<&'static str>,
}

impl Listing {
    /// The files without lines.
    /// The files without lines, as many as `max_output` lets through.
    pub fn without_lines(files: &[Found], limits: &Limits) -> Self {
        let mut listing = Self {
            files: vec![Listed::NoLines; files.len()],
            cut: vec![false; files.len()],
            until: vec![None; files.len()],
            shown: files.len(),
            truncated_by: Vec::new(),
        };
        listing.limit_output(files, limits, None);
        listing
    }

    /// The lines of the files in `lines` detail: the fragments which are not on repeated lines, the first
    /// `max_matches_per_file` of every file, those before the line which makes its output longer than
    /// `max_output_per_file` (its blocks made with `options`), then of those the ones of the first `max_lines` matched
    /// lines and the first `max_matches` fragments in the order of the files.
    pub fn new(
        files: &[Found],
        repeated: &Repeated,
        limits: &Limits,
        options: &BlockOptions,
    ) -> Self {
        let mut lines_left = limits.max_lines;
        let mut matches_left = limits.max_matches;
        let (mut cut_lines, mut cut_matches, mut matches_per_file) = (false, false, false);
        let mut listed = Vec::with_capacity(files.len());
        let mut cut = vec![false; files.len()];
        let mut until = vec![None; files.len()];
        for (index, file) in files.iter().enumerate() {
            let Some(m) = file.matches.as_ref().filter(|m| !m.fragments.is_empty()) else {
                listed.push(Listed::NoLines);
                continue;
            };
            let mut rest = repeated.rest(index, &m.content, &m.fragments);
            if rest.is_empty() {
                listed.push(Listed::LeftOut);
                continue;
            }
            if rest.len() > limits.max_matches_per_file {
                cut[index] = true;
                matches_per_file = true;
                rest.truncate(limits.max_matches_per_file);
            }
            until[index] = first_over(&m.content, &rest, options, limits);
            if let Some(line) = until[index] {
                cut[index] = true;
                rest.retain(|f| last_line(&m.content, f) < line);
            }
            let lines = lines_of(&m.content, &rest);
            let rest = if lines.len() > lines_left {
                cut_lines = true;
                first_lines(&m.content, rest, &lines, lines_left)
            } else {
                rest
            };
            let mut rest = rest;
            if rest.len() > matches_left {
                cut_matches = true;
                rest.truncate(matches_left);
            }
            lines_left = lines_left.saturating_sub(lines_of(&m.content, &rest).len());
            matches_left -= rest.len();
            listed.push(Listed::Lines(rest));
        }
        let mut truncated_by = Vec::new();
        if cut_lines {
            truncated_by.push("max_lines");
        }
        if cut_matches {
            truncated_by.push("max_matches");
        }
        if matches_per_file {
            truncated_by.push("max_matches_per_file");
        }
        if until.iter().any(Option::is_some) {
            truncated_by.push("max_output_per_file");
        }
        let mut listing = Self {
            files: listed,
            cut,
            until,
            shown: files.len(),
            truncated_by,
        };
        listing.limit_output(files, limits, Some(options));
        listing
    }

    /// Adds up the characters of the full paths of the files and of their lines (blocks made with `options`, none
    /// without): the file whose path goes over `max_output` is not written, the line which goes over is not listed,
    /// and nothing after them.
    fn limit_output(&mut self, files: &[Found], limits: &Limits, options: Option<&BlockOptions>) {
        let mut used = 0_usize;
        for (index, file) in files.iter().enumerate() {
            if self.files[index] == Listed::LeftOut {
                continue;
            }
            used += display_found(file).chars().count();
            if used > limits.max_output {
                self.shown = index;
                self.truncated_by.push("max_output");
                return;
            }
            let (Some(options), Some(m), Listed::Lines(fragments)) =
                (options, &file.matches, &mut self.files[index])
            else {
                continue;
            };
            let lines = shown_lines(&m.content, fragments, options, limits, self.until[index]);
            for (line, chars) in lines {
                used += chars;
                if used > limits.max_output {
                    self.until[index] = Some(line);
                    fragments.retain(|f| last_line(&m.content, f) < line);
                    self.shown = index + 1;
                    self.truncated_by.push("max_output");
                    return;
                }
            }
        }
    }
}

/// The first line of the blocks of the fragments which makes their output longer than `max_output_per_file`
/// characters.
fn first_over(
    content: &Content,
    fragments: &[Fragment],
    options: &BlockOptions,
    limits: &Limits,
) -> Option<usize> {
    let mut used = 0_usize;
    shown_lines(content, fragments, options, limits, None)
        .into_iter()
        .find(|(_, chars)| {
            used += chars;
            used > limits.max_output_per_file
        })
        .map(|(line, _)| line)
}

/// The lines of the blocks of the fragments before the line `until` with their characters as they are shown: after
/// the dedent and the cut of long lines, without the line numbers and markers.
fn shown_lines(
    content: &Content,
    fragments: &[Fragment],
    options: &BlockOptions,
    limits: &Limits,
    until: Option<usize>,
) -> Vec<(usize, usize)> {
    before(blocks(content, fragments, options), until)
        .iter()
        .flat_map(|block| {
            block.lines.iter().map(|line| {
                let (text, _) = cut_line(
                    content,
                    fragments,
                    line.index,
                    block.indent,
                    Highlight::None,
                    limits.max_line_length,
                );
                (line.index, text.chars().count())
            })
        })
        .collect()
}

/// The last line a fragment touches.
fn last_line(content: &Content, f: &Fragment) -> usize {
    content.line_of(f.range.end.saturating_sub(1).max(f.range.start))
}

/// The fragments of the first `count` of the matched `lines`: a multiline fragment only with all its lines.
fn first_lines(
    content: &Content,
    fragments: Vec<Fragment>,
    lines: &[usize],
    count: usize,
) -> Vec<Fragment> {
    let Some(&last) = count.checked_sub(1).and_then(|i| lines.get(i)) else {
        return Vec::new();
    };
    fragments
        .into_iter()
        .filter(|f| last_line(content, f) <= last)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::Content;
    use crate::output::BlockOptions;
    use crate::query::ast::CondId;
    use crate::search::FileMatches;

    fn file(text: &str) -> Found {
        let content = Content::from_bytes(text.as_bytes());
        let fragments: Vec<Fragment> = content
            .text()
            .match_indices("x\ny")
            .chain(content.text().match_indices('a'))
            .map(|(i, m)| Fragment {
                range: i..i + m.len(),
                ids: vec![(1, CondId::Number(1))],
                escape: None,
            })
            .collect();
        let fragments = crate::output::merge_identical(fragments);
        let lines = crate::output::matched_lines(&content, &fragments);
        Found {
            rel: "f".into(),
            member: None,
            is_dir: false,
            conditions: vec![],
            matches: Some(FileMatches {
                content,
                fragments,
                lines,
            }),
            encoding: None,
            id: None,
            path_fragments: None,
            meta: None,
            signals: None,
            score: None,
        }
    }

    fn listed_lines(listing: &Listing, files: &[Found]) -> Vec<Vec<usize>> {
        listing
            .files
            .iter()
            .zip(files)
            .map(|(listed, file)| match listed {
                Listed::Lines(fragments) => {
                    let m = file.matches.as_ref().unwrap();
                    crate::output::lines::lines_of(&m.content, fragments)
                }
                _ => Vec::new(),
            })
            .collect()
    }

    #[test]
    fn the_first_matched_lines() {
        // lines 0, 1 and 3 (the multiline fragment x/y touches 1 and 2)
        let files = [file("a\na\nb\na\n"), file("a\nx\ny\na\n")];
        let limits = Limits {
            max_lines: 4,
            ..Limits::default()
        };
        let listing = Listing::new(
            &files,
            &Repeated::default(),
            &limits,
            &BlockOptions::default(),
        );
        assert_eq!(listed_lines(&listing, &files), [vec![0, 1, 3], vec![0]]);
        assert_eq!(listing.truncated_by, ["max_lines"]);
        let limits = Limits {
            max_lines: 5,
            ..Limits::default()
        };
        // the multiline fragment needs lines 1 and 2: it is left out with line 3
        let listing = Listing::new(
            &files,
            &Repeated::default(),
            &limits,
            &BlockOptions::default(),
        );
        assert_eq!(listed_lines(&listing, &files), [vec![0, 1, 3], vec![0]]);
        let all = Listing::new(
            &files,
            &Repeated::default(),
            &Limits::default(),
            &BlockOptions::default(),
        );
        assert_eq!(
            listed_lines(&all, &files),
            [vec![0, 1, 3], vec![0, 1, 2, 3]]
        );
        assert!(all.truncated_by.is_empty());
        let one = Listing::new(
            &files,
            &Repeated::default(),
            &Limits {
                max_lines: 1,
                ..Limits::default()
            },
            &BlockOptions::default(),
        );
        assert_eq!(listed_lines(&one, &files), [vec![0], vec![]]);
        assert!(matches!(one.files[1], Listed::Lines(ref f) if f.is_empty()));
    }

    #[test]
    fn the_first_matches() {
        // three fragments on line 0, one on line 1; the multiline fragment of the second file is one match
        let files = [file("aaa\na\n"), file("x\ny\na\n")];
        let listing = Listing::new(
            &files,
            &Repeated::default(),
            &Limits {
                max_matches: 2,
                ..Limits::default()
            },
            &BlockOptions::default(),
        );
        assert!(matches!(&listing.files[0], Listed::Lines(f) if f.len() == 2));
        assert_eq!(listed_lines(&listing, &files), [vec![0], vec![]]);
        assert_eq!(listing.truncated_by, ["max_matches"]);
        let both = Listing::new(
            &files,
            &Repeated::default(),
            &Limits {
                max_lines: 3,
                max_matches: 5,
                ..Limits::default()
            },
            &BlockOptions::default(),
        );
        assert_eq!(listed_lines(&both, &files), [vec![0, 1], vec![]]);
        assert_eq!(both.truncated_by, ["max_lines"]);
        let matches_left = Listing::new(
            &files,
            &Repeated::default(),
            &Limits {
                max_matches: 5,
                ..Limits::default()
            },
            &BlockOptions::default(),
        );
        assert_eq!(
            listed_lines(&matches_left, &files),
            [vec![0, 1], vec![0, 1]]
        );
    }

    #[test]
    fn the_first_matches_of_every_file() {
        let files = [file("aaa\na\n"), file("a\n"), file("aa\n")];
        let limits = Limits {
            max_matches_per_file: 2,
            max_matches: 3,
            ..Limits::default()
        };
        let listing = Listing::new(
            &files,
            &Repeated::default(),
            &limits,
            &BlockOptions::default(),
        );
        assert_eq!(listing.cut, [true, false, false]);
        assert_eq!(listed_lines(&listing, &files), [vec![0], vec![0], vec![]]);
        assert_eq!(
            listing.truncated_by,
            ["max_matches", "max_matches_per_file"]
        );
        let per_file = Listing::new(
            &files,
            &Repeated::default(),
            &Limits {
                max_matches_per_file: 2,
                ..Limits::default()
            },
            &BlockOptions::default(),
        );
        assert_eq!(per_file.truncated_by, ["max_matches_per_file"]);
    }

    #[test]
    fn the_first_characters_of_every_file() {
        let files = [file("aaaa\naaaa\naaaa\n"), file("a\nbbbb\na\n")];
        let limits = Limits {
            max_output_per_file: 9,
            ..Limits::default()
        };
        let listing = Listing::new(
            &files,
            &Repeated::default(),
            &limits,
            &BlockOptions::default(),
        );
        assert_eq!(listed_lines(&listing, &files), [vec![0, 1], vec![0, 2]]);
        assert_eq!(listing.until, [Some(2), None]);
        assert_eq!(listing.cut, [true, false]);
        assert_eq!(listing.truncated_by, ["max_output_per_file"]);
        // the context line of four characters counts
        let context = BlockOptions {
            after: 1,
            ..BlockOptions::default()
        };
        let limits = Limits {
            max_output_per_file: 5,
            ..Limits::default()
        };
        let listing = Listing::new(&files[1..], &Repeated::default(), &limits, &context);
        assert_eq!(listed_lines(&listing, &files[1..]), [vec![0]]);
        assert_eq!(listing.until, [Some(2)]);
    }

    #[test]
    fn the_first_characters_of_the_response() {
        // the path `f` is one character, every line four
        let files = [file("aaaa\naaaa\n"), file("aaaa\n")];
        let limits = |max_output: usize| Limits {
            max_output,
            ..Limits::default()
        };
        let listing = Listing::new(
            &files,
            &Repeated::default(),
            &limits(6),
            &BlockOptions::default(),
        );
        assert_eq!(listing.shown, 1);
        assert_eq!(listed_lines(&listing, &files)[0], [0]);
        assert_eq!(listing.until[0], Some(1));
        assert_eq!(listing.truncated_by, ["max_output"]);
        let listing = Listing::new(
            &files,
            &Repeated::default(),
            &limits(10),
            &BlockOptions::default(),
        );
        assert_eq!(listing.shown, 2);
        assert_eq!(listed_lines(&listing, &files), [vec![0, 1], vec![]]);
        let all = Listing::new(
            &files,
            &Repeated::default(),
            &limits(14),
            &BlockOptions::default(),
        );
        assert_eq!(all.shown, 2);
        assert!(all.truncated_by.is_empty());
        let paths = Listing::without_lines(&files, &limits(1));
        assert_eq!(paths.shown, 1);
        assert_eq!(Listing::without_lines(&files, &limits(0)).shown, 0);
    }
}
