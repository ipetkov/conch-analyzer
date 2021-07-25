use crate::visit::{self, Visitor};
use conch_parser::ast;
use std::collections::HashMap;

type ParseResult<T> = conch_parser::parse::ParseResult<T, void::Void>;

#[derive(Default)]
struct Histogram {
    map: HashMap<String, usize>,
}

impl Histogram {
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

pub(crate) fn histogram<I>(cmds: I) -> ParseResult<HashMap<String, usize>>
where
    I: IntoIterator<Item = ParseResult<ast::TopLevelCommand<String>>>,
{
    let mut histogram = Histogram::new();

    let mut cmd_visitor = CmdVisitor {
        histogram: &mut histogram,
    };

    for cmd in cmds {
        cmd_visitor.visit_top_level_command(&cmd?)
    }

    Ok(histogram.map)
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

#[cfg(test)]
mod test {
    use conch_parser::lexer::Lexer;
    use conch_parser::parse::DefaultParser;
    use super::histogram;
    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    #[test]
    fn histogram_smoke() {
        let s = r#"
            single_command ignore ignore ignore

            and_cmd ignore && and_cmd ignore also ignore
            or_cmd ignore || or_cmd ignore also ignore

            job ignore ignore &

            pipe ignore | pipe | pipe also ignore

            function foo() {
                fn_body
                fn_body
            }

            {
                brace ignore
                brace ignore ignore
                brace ignore ignore ignore
            }

            (
                subshell ignore
                subshell ignore ignore
            )

            while while_guard1; while_guard2; do
                while_body1
                while_body2
            done

            until until_guard1; until_guard2; do
                until_body1
                until_body2
            done

            if if_guard1; if_guard1; then
                if_body1
            elif if_guard2; if_guard2; then
                if_body2
                if_body2
            else
                els_body
                els_body
            fi

            for f in ignore ignore ignore; do
                for_body ignore
                for_body $f
            done

            case x in
                foo) case_foo;;
                bar) case_bar;;
            esac

            # words
            hello'world' ignore ignore
            hello"world" ignore ignore
            hello'foo'"bar"\b\a\z ignore ignore

            * ignore
            ? ignore
            [ ignore
            ] ignore
            ~ ignore
            : ignore

            echo $(subst1 ignore)
            echo ${param:-$(subst2 ignore)}
            echo ${param:=$(subst3 ignore)}
            echo ${param:?$(subst4 ignore)}
            echo ${param:+$(subst5 ignore)}
            echo ${param%$(subst6 ignore)}
            echo ${param%%$(subst7 ignore)}
            echo ${param#$(subst8 ignore)}
            echo ${param##$(subst9 ignore)}

            # Nested subst
            echo $(subst10 ignore $(subst11 ignore) $(subst12 ignore $(subst13 ignore)))

            # Redirects
            echo <$(redsub) >$(redsub) <>$(redsub) >>$(redsub) >|$(redsub) <<EOF
                $(redsub)
            EOF
        "#;
        let lex = Lexer::new(s.chars());
        let parser = DefaultParser::new(lex);

        let mut expected = BTreeMap::new();
        expected.insert("single_command".into(), 1);
        expected.insert("or_cmd".into(), 2);
        expected.insert("and_cmd".into(), 2);
        expected.insert("job".into(), 1);
        expected.insert("pipe".into(), 3);
        expected.insert("fn_body".into(), 2);
        expected.insert("brace".into(), 3);
        expected.insert("subshell".into(), 2);
        expected.insert("while_guard1".into(), 1);
        expected.insert("while_guard2".into(), 1);
        expected.insert("while_body1".into(), 1);
        expected.insert("while_body2".into(), 1);
        expected.insert("until_guard1".into(), 1);
        expected.insert("until_guard2".into(), 1);
        expected.insert("until_body1".into(), 1);
        expected.insert("until_body2".into(), 1);
        expected.insert("if_guard1".into(), 2);
        expected.insert("if_body1".into(), 1);
        expected.insert("if_guard2".into(), 2);
        expected.insert("if_body2".into(), 2);
        expected.insert("els_body".into(), 2);
        expected.insert("for_body".into(), 2);
        expected.insert("case_foo".into(), 1);
        expected.insert("case_bar".into(), 1);
        expected.insert("helloworld".into(), 2);
        expected.insert("hellofoobarbaz".into(), 1);
        expected.insert("*".into(), 1);
        expected.insert("?".into(), 1);
        expected.insert("[".into(), 1);
        expected.insert("]".into(), 1);
        expected.insert("~".into(), 1);
        expected.insert(":".into(), 1);
        expected.insert("echo".into(), 11);
        expected.insert("subst1".into(), 1);
        expected.insert("subst2".into(), 1);
        expected.insert("subst3".into(), 1);
        expected.insert("subst4".into(), 1);
        expected.insert("subst5".into(), 1);
        expected.insert("subst6".into(), 1);
        expected.insert("subst7".into(), 1);
        expected.insert("subst8".into(), 1);
        expected.insert("subst9".into(), 1);
        expected.insert("subst10".into(), 1);
        expected.insert("subst11".into(), 1);
        expected.insert("subst12".into(), 1);
        expected.insert("subst13".into(), 1);
        expected.insert("redsub".into(), 6);

        let result = histogram(parser)
            .expect("parse failed");

        assert_eq!(expected, BTreeMap::from_iter(result));
    }
}
