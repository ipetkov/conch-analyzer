use crate::visit::{self, Visitor};
use conch_parser::ast;
use std::collections::HashMap;

type ParseResult<T> = conch_parser::parse::ParseResult<T, void::Void>;

#[derive(Default)]
pub(crate) struct Histogram {
    map: HashMap<String, usize>,
}

impl Histogram {
    pub(crate) fn histogram<I>(cmds: I) -> ParseResult<HashMap<String, usize>>
    where
        I: IntoIterator<Item = ParseResult<ast::TopLevelCommand<String>>>,
    {
        let mut histogram = Self::new();

        let mut cmd_visitor = CmdVisitor {
            histogram: &mut histogram,
        };

        for cmd in cmds {
            cmd_visitor.visit_top_level_command(&cmd?)
        }

        Ok(histogram.map)
    }

    fn new() -> Self {
        Self::default()
    }

    fn add_cmd(&mut self, cmd: String) {
        let value = self.map.entry(cmd).or_insert(0);

        *value += 1;
    }
}

/// A visitor which extracts all SimpleCommands that can be found
/// (from all places including substitutions etc.)
struct CmdVisitor<'a> {
    histogram: &'a mut Histogram,
}

impl<'ast> Visitor<'ast> for CmdVisitor<'_> {
    fn visit_simple_command(&mut self, cmd: &'ast ast::DefaultSimpleCommand) {
        // Default behavior, keep walking the tree
        visit::walk_simple_command(self, cmd);

        self.histogram.visit_simple_command(cmd);
    }
}

fn simple_word_to_str(word: &ast::DefaultSimpleWord) -> Option<&str> {
    let name = match word {
        ast::SimpleWord::Literal(l) => l,
        ast::SimpleWord::Escaped(l) => l,
        ast::SimpleWord::Star => "*",
        ast::SimpleWord::Question => "?",
        ast::SimpleWord::SquareOpen => "[",
        ast::SimpleWord::SquareClose => "]",
        ast::SimpleWord::Tilde => "~",
        ast::SimpleWord::Colon => ":",

        // Skip these as we aren't emulating runtime execution
        ast::SimpleWord::Param(_) | ast::SimpleWord::Subst(_) => return None,
    };

    Some(name)
}

impl<'ast> Visitor<'ast> for Histogram {
    fn visit_simple_command(&mut self, cmd: &'ast ast::DefaultSimpleCommand) {
        let name = cmd
            .redirects_or_cmd_words
            .iter()
            .find_map(|roev| match roev {
                ast::RedirectOrCmdWord::Redirect(..) => None,
                ast::RedirectOrCmdWord::CmdWord(w) => Some(w),
            });

        if let Some(name) = name {
            self.visit_top_level_word(name);
        }
    }

    fn visit_concat_word(&mut self, words: &'ast [ast::DefaultWord]) {
        let mut concat = String::new();

        for word in words {
            let next = match word {
                ast::DefaultWord::Simple(s) => match simple_word_to_str(s) {
                    Some(s) => s,
                    None => return, // Can't emulate execution, bail
                },

                ast::DefaultWord::SingleQuoted(s) => s,

                ast::DefaultWord::DoubleQuoted(d) => {
                    for s in d {
                        match simple_word_to_str(s) {
                            Some(s) => concat.push_str(s),
                            None => return, // Can't emulate execution, bail
                        }
                    }
                    continue;
                }
            };

            concat.push_str(next);
        }

        self.add_cmd(concat);
    }

    fn visit_simple_word(&mut self, word: &'ast ast::DefaultSimpleWord) {
        if let Some(name) = simple_word_to_str(word) {
            self.add_cmd(name.to_owned());
        }
    }

    fn visit_single_quoted(&mut self, word: &'ast str) {
        self.add_cmd(word.to_owned());
    }

    fn visit_double_quoted(&mut self, words: &'ast [ast::DefaultSimpleWord]) {
        let mut concat = String::new();
        for s in words {
            match simple_word_to_str(s) {
                Some(s) => concat.push_str(s),
                None => return, // Can't emulate execution, bail
            }
        }

        self.add_cmd(concat);
    }
}
