use conch_parser::ast;

pub trait Visitor<'ast>: Sized {
    fn visit_top_level_command(&mut self, cmd: &'ast ast::TopLevelCommand<String>) {
        walk_top_level_command(self, cmd);
    }

    fn visit_command(&mut self, cmd: &'ast ast::DefaultCommand) {
        walk_command(self, cmd);
    }

    fn visit_command_job(&mut self, and_or_list: &'ast ast::DefaultAndOrList) {
        walk_and_or_list(self, and_or_list);
    }

    fn visit_and_or_list(&mut self, and_or_list: &'ast ast::DefaultAndOrList) {
        walk_and_or_list(self, and_or_list);
    }

    fn visit_and_or(&mut self, cmd: &'ast ast::AndOr<ast::DefaultListableCommand>) {
        walk_and_or(self, cmd);
    }

    fn visit_and(&mut self, cmd: &'ast ast::DefaultListableCommand) {
        walk_listable_command(self, cmd);
    }

    fn visit_or(&mut self, cmd: &'ast ast::DefaultListableCommand) {
        walk_listable_command(self, cmd);
    }

    fn visit_listable_command(&mut self, cmd: &'ast ast::DefaultListableCommand) {
        walk_listable_command(self, cmd);
    }

    fn visit_pipe(&mut self, invert: bool, body: &'ast [ast::DefaultPipeableCommand]) {
        walk_pipe(self, invert, body);
    }

    fn visit_pipeable_command(&mut self, cmd: &'ast ast::DefaultPipeableCommand) {
        walk_pipeable_command(self, cmd);
    }

    fn visit_compound_command(
        &mut self,
        kind: &'ast ast::DefaultCompoundCommandKind,
        io: &'ast [ast::DefaultRedirect],
    ) {
        walk_compound_command(self, kind, io);
    }

    fn visit_compound_command_kind(&mut self, kind: &'ast ast::DefaultCompoundCommandKind) {
        walk_compound_command_kind(self, kind);
    }

    fn visit_func_def(&mut self, name: &'ast str, body: &'ast ast::DefaultCompoundCommand) {
        walk_func_def(self, name, body);
    }

    fn visit_simple_command(&mut self, cmd: &'ast ast::DefaultSimpleCommand) {
        walk_simple_command(self, cmd);
    }

    fn visit_brace_command(&mut self, cmds: &'ast [ast::TopLevelCommand<String>]) {
        walk_brace_command(self, cmds);
    }

    fn visit_subshell(&mut self, cmds: &'ast [ast::TopLevelCommand<String>]) {
        walk_subshell(self, cmds);
    }

    fn visit_while_loop(&mut self, gbp: &'ast ast::GuardBodyPair<ast::TopLevelCommand<String>>) {
        walk_guard_body_pair(self, gbp);
    }

    fn visit_until_loop(&mut self, gbp: &'ast ast::GuardBodyPair<ast::TopLevelCommand<String>>) {
        walk_guard_body_pair(self, gbp);
    }

    fn visit_guard_body_pair(
        &mut self,
        gbp: &'ast ast::GuardBodyPair<ast::TopLevelCommand<String>>,
    ) {
        walk_guard_body_pair(self, gbp);
    }

    fn visit_guard(&mut self, cmd: &'ast [ast::TopLevelCommand<String>]) {
        walk_top_level_command_array(self, cmd);
    }

    fn visit_guard_body(&mut self, cmd: &'ast [ast::TopLevelCommand<String>]) {
        walk_top_level_command_array(self, cmd);
    }

    fn visit_if(
        &mut self,
        conditionals: &'ast [ast::GuardBodyPair<ast::TopLevelCommand<String>>],
        else_branch: Option<&'ast [ast::TopLevelCommand<String>]>,
    ) {
        walk_if(self, conditionals, else_branch);
    }

    fn visit_for(
        &mut self,
        var: &'ast str,
        words: Option<&'ast [ast::TopLevelWord<String>]>,
        body: &'ast [ast::TopLevelCommand<String>],
    ) {
        walk_for(self, var, words, body);
    }

    fn visit_case(
        &mut self,
        word: &'ast ast::TopLevelWord<String>,
        arms: &'ast [ast::PatternBodyPair<
            ast::TopLevelWord<String>,
            ast::TopLevelCommand<String>,
        >],
    ) {
        walk_case(self, word, arms);
    }

    fn visit_case_arms(
        &mut self,
        arms: &'ast [ast::PatternBodyPair<
            ast::TopLevelWord<String>,
            ast::TopLevelCommand<String>,
        >],
    ) {
        walk_case_arms(self, arms);
    }

    fn visit_pattern_body_pair(
        &mut self,
        pbp: &'ast ast::PatternBodyPair<ast::TopLevelWord<String>, ast::TopLevelCommand<String>>,
    ) {
        walk_pattern_body_pair(self, pbp);
    }

    fn visit_case_patterns(&mut self, pattern: &'ast [ast::TopLevelWord<String>]) {
        walk_case_patterns(self, pattern);
    }

    fn visit_case_pattern_body(&mut self, body: &'ast [ast::TopLevelCommand<String>]) {
        walk_case_pattern_body(self, body);
    }

    fn visit_env_var(&mut self, name: &'ast str, value: &'ast Option<ast::TopLevelWord<String>>) {
        walk_env_var(self, name, value);
    }

    fn visit_var_assignment(
        &mut self,
        name: &'ast str,
        value: &'ast Option<ast::TopLevelWord<String>>,
    ) {
        walk_var_assignment(self, name, value);
    }

    fn visit_top_level_word(&mut self, word: &'ast ast::TopLevelWord<String>) {
        walk_top_level_word(self, word);
    }

    fn visit_complex_word(&mut self, word: &'ast ast::DefaultComplexWord) {
        walk_complex_word(self, word);
    }

    fn visit_word(&mut self, word: &'ast ast::DefaultWord) {
        walk_word(self, word);
    }

    fn visit_concat_word(&mut self, words: &'ast [ast::DefaultWord]) {
        walk_concat_word(self, words);
    }

    fn visit_simple_word(&mut self, word: &'ast ast::DefaultSimpleWord) {
        walk_simple_word(self, word);
    }

    fn visit_single_quoted(&mut self, word: &'ast str) {
        let _word = word;
    }

    fn visit_double_quoted(&mut self, words: &'ast [ast::DefaultSimpleWord]) {
        walk_double_quoted(self, words);
    }

    fn visit_literal_word(&mut self, lit: &'ast str) {
        let _lit = lit;
    }

    fn visit_esaped_word(&mut self, esc: &'ast str) {
        let _esc = esc;
    }

    fn visit_param(&mut self, param: &'ast ast::DefaultParameter) {
        let _param = param;
    }

    fn visit_param_subst(&mut self, subst: &'ast ast::DefaultParameterSubstitution) {
        walk_param_subst(self, subst);
    }

    fn visit_cmd_subst(&mut self, cmds: &'ast [ast::TopLevelCommand<String>]) {
        walk_top_level_command_array(self, cmds);
    }

    fn visit_param_subst_len(&mut self, param: &'ast ast::DefaultParameter) {
        self.visit_param(param);
    }

    fn visit_param_subst_arith(&mut self, arith: &'ast Option<ast::DefaultArithmetic>) {
        let _arith = arith;
        // FIXME: walk this
    }

    fn visit_param_subst_default(
        &mut self,
        strict: bool,
        param: &'ast ast::DefaultParameter,
        default: &'ast Option<ast::TopLevelWord<String>>,
    ) {
        walk_word_substitution(self, strict, param, default);
    }

    fn visit_param_subst_assign(
        &mut self,
        strict: bool,
        param: &'ast ast::DefaultParameter,
        assign: &'ast Option<ast::TopLevelWord<String>>,
    ) {
        walk_word_substitution(self, strict, param, assign);
    }

    fn visit_param_subst_error(
        &mut self,
        strict: bool,
        param: &'ast ast::DefaultParameter,
        error: &'ast Option<ast::TopLevelWord<String>>,
    ) {
        walk_word_substitution(self, strict, param, error);
    }

    fn visit_param_subst_alternative(
        &mut self,
        strict: bool,
        param: &'ast ast::DefaultParameter,
        alternative: &'ast Option<ast::TopLevelWord<String>>,
    ) {
        walk_word_substitution(self, strict, param, alternative);
    }

    fn visit_param_subst_remove_smallest_suffix(
        &mut self,
        param: &'ast ast::DefaultParameter,
        smallest_suffix: &'ast Option<ast::TopLevelWord<String>>,
    ) {
        walk_word_substitution_removal(self, param, smallest_suffix);
    }

    fn visit_param_subst_remove_largest_suffix(
        &mut self,
        param: &'ast ast::DefaultParameter,
        largest_suffix: &'ast Option<ast::TopLevelWord<String>>,
    ) {
        walk_word_substitution_removal(self, param, largest_suffix);
    }

    fn visit_param_subst_remove_smallest_prefix(
        &mut self,
        param: &'ast ast::DefaultParameter,
        smallest_prefix: &'ast Option<ast::TopLevelWord<String>>,
    ) {
        walk_word_substitution_removal(self, param, smallest_prefix);
    }

    fn visit_param_subst_remove_largest_prefix(
        &mut self,
        param: &'ast ast::DefaultParameter,
        largest_prefix: &'ast Option<ast::TopLevelWord<String>>,
    ) {
        walk_word_substitution_removal(self, param, largest_prefix);
    }

    fn visit_star_word(&mut self) {}

    fn visit_question_word(&mut self) {}

    fn visit_square_open_word(&mut self) {}

    fn visit_square_close_word(&mut self) {}

    fn visit_tilde_word(&mut self) {}

    fn visit_colon_word(&mut self) {}

    fn visit_redirect(&mut self, redirect: &'ast ast::DefaultRedirect) {
        walk_redirect(self, redirect);
    }

    fn visit_redirect_read(&mut self, n: &'ast Option<u16>, file: &'ast ast::TopLevelWord<String>) {
        walk_redirect_read(self, n, file);
    }

    fn visit_redirect_write(
        &mut self,
        n: &'ast Option<u16>,
        file: &'ast ast::TopLevelWord<String>,
    ) {
        walk_redirect_write(self, n, file);
    }

    fn visit_redirect_readwrite(
        &mut self,
        n: &'ast Option<u16>,
        file: &'ast ast::TopLevelWord<String>,
    ) {
        walk_redirect_readwrite(self, n, file);
    }

    fn visit_redirect_append(
        &mut self,
        n: &'ast Option<u16>,
        file: &'ast ast::TopLevelWord<String>,
    ) {
        walk_redirect_append(self, n, file);
    }

    fn visit_redirect_clobber(
        &mut self,
        n: &'ast Option<u16>,
        file: &'ast ast::TopLevelWord<String>,
    ) {
        walk_redirect_clobber(self, n, file);
    }

    fn visit_redirect_heredoc(
        &mut self,
        n: &'ast Option<u16>,
        file: &'ast ast::TopLevelWord<String>,
    ) {
        walk_redirect_heredoc(self, n, file);
    }

    fn visit_redirect_dup_read(
        &mut self,
        n: &'ast Option<u16>,
        file: &'ast ast::TopLevelWord<String>,
    ) {
        walk_redirect_dup_read(self, n, file);
    }

    fn visit_redirect_dup_write(
        &mut self,
        n: &'ast Option<u16>,
        file: &'ast ast::TopLevelWord<String>,
    ) {
        walk_redirect_dup_write(self, n, file);
    }
}

pub fn walk_top_level_command<'a, V: Visitor<'a>>(
    v: &mut V,
    cmd: &'a ast::TopLevelCommand<String>,
) {
    let ast::TopLevelCommand(ref inner) = cmd;
    v.visit_command(inner);
}

pub fn walk_top_level_command_array<'a, V: Visitor<'a>>(
    v: &mut V,
    cmds: &'a [ast::TopLevelCommand<String>],
) {
    for cmd in cmds {
        let ast::TopLevelCommand(ref inner) = cmd;
        v.visit_command(inner);
    }
}

pub fn walk_command<'a, V: Visitor<'a>>(v: &mut V, cmd: &'a ast::DefaultCommand) {
    match cmd {
        ast::Command::Job(j) => v.visit_command_job(j),
        ast::Command::List(l) => v.visit_and_or_list(l),
    }
}

pub fn walk_and_or_list<'a, V: Visitor<'a>>(v: &mut V, and_or_list: &'a ast::DefaultAndOrList) {
    let ast::DefaultAndOrList { first, rest } = and_or_list;
    v.visit_listable_command(first);

    for cmd in rest {
        v.visit_and_or(cmd);
    }
}

pub fn walk_and_or<'a, V: Visitor<'a>>(
    v: &mut V,
    cmd: &'a ast::AndOr<ast::DefaultListableCommand>,
) {
    match cmd {
        ast::AndOr::And(and) => v.visit_and(and),
        ast::AndOr::Or(or) => v.visit_and(or),
    }
}

pub fn walk_listable_command<'a, V: Visitor<'a>>(v: &mut V, cmd: &'a ast::DefaultListableCommand) {
    match cmd {
        ast::ListableCommand::Pipe(invert, body) => v.visit_pipe(*invert, body),
        ast::ListableCommand::Single(cmd) => v.visit_pipeable_command(cmd),
    }
}

pub fn walk_pipe<'a, V: Visitor<'a>>(
    v: &mut V,
    invert: bool,
    body: &'a [ast::DefaultPipeableCommand],
) {
    let _invert = invert;
    for cmd in body {
        v.visit_pipeable_command(cmd);
    }
}

pub fn walk_pipeable_command<'a, V: Visitor<'a>>(v: &mut V, cmd: &'a ast::DefaultPipeableCommand) {
    match cmd {
        ast::DefaultPipeableCommand::Simple(s) => v.visit_simple_command(s),
        ast::DefaultPipeableCommand::Compound(c) => {
            let ast::CompoundCommand { kind, io } = &**c;
            v.visit_compound_command(kind, io);
        }
        ast::DefaultPipeableCommand::FunctionDef(name, body) => v.visit_func_def(name, &*body),
    }
}

pub fn walk_func_def<'a, V: Visitor<'a>>(
    v: &mut V,
    name: &'a str,
    body: &'a ast::DefaultCompoundCommand,
) {
    let _name = name;
    let ast::CompoundCommand { kind, io } = body;
    v.visit_compound_command(kind, io);
}

pub fn walk_brace_command<'a, V: Visitor<'a>>(v: &mut V, cmds: &'a [ast::TopLevelCommand<String>]) {
    walk_top_level_command_array(v, cmds);
}

pub fn walk_subshell<'a, V: Visitor<'a>>(v: &mut V, cmds: &'a [ast::TopLevelCommand<String>]) {
    walk_top_level_command_array(v, cmds);
}

pub fn walk_if<'a, V: Visitor<'a>>(
    v: &mut V,
    conditionals: &'a [ast::GuardBodyPair<ast::TopLevelCommand<String>>],
    else_branch: Option<&'a [ast::TopLevelCommand<String>]>,
) {
    for conditional in conditionals {
        v.visit_guard_body_pair(conditional);
    }

    if let Some(els) = else_branch {
        walk_top_level_command_array(v, els);
    }
}

pub fn walk_for<'a, V: Visitor<'a>>(
    v: &mut V,
    var: &'a str,
    words: Option<&'a [ast::TopLevelWord<String>]>,
    body: &'a [ast::TopLevelCommand<String>],
) {
    let _var = var;

    if let Some(words) = words {
        walk_top_level_word_array(v, words)
    }

    walk_top_level_command_array(v, body)
}

pub fn walk_guard_body_pair<'a, V: Visitor<'a>>(
    v: &mut V,
    gbp: &'a ast::GuardBodyPair<ast::TopLevelCommand<String>>,
) {
    let ast::GuardBodyPair { guard, body } = gbp;
    v.visit_guard(guard);
    v.visit_guard_body(body);
}

pub fn walk_case<'a, V: Visitor<'a>>(
    v: &mut V,
    word: &'a ast::TopLevelWord<String>,
    arms: &'a [ast::PatternBodyPair<ast::TopLevelWord<String>, ast::TopLevelCommand<String>>],
) {
    v.visit_top_level_word(word);
    v.visit_case_arms(arms);
}

pub fn walk_case_arms<'a, V: Visitor<'a>>(
    v: &mut V,
    arms: &'a [ast::PatternBodyPair<ast::TopLevelWord<String>, ast::TopLevelCommand<String>>],
) {
    for pbp in arms {
        v.visit_pattern_body_pair(pbp);
    }
}

pub fn walk_pattern_body_pair<'a, V: Visitor<'a>>(
    v: &mut V,
    pbp: &'a ast::PatternBodyPair<ast::TopLevelWord<String>, ast::TopLevelCommand<String>>,
) {
    let ast::PatternBodyPair { patterns, body } = pbp;

    v.visit_case_patterns(patterns);
    v.visit_case_pattern_body(body);
}

pub fn walk_case_patterns<'a, V: Visitor<'a>>(
    v: &mut V,
    patterns: &'a [ast::TopLevelWord<String>],
) {
    walk_top_level_word_array(v, patterns);
}

pub fn walk_case_pattern_body<'a, V: Visitor<'a>>(
    v: &mut V,
    patterns: &'a [ast::TopLevelCommand<String>],
) {
    walk_top_level_command_array(v, patterns);
}

pub fn walk_simple_command<'a, V: Visitor<'a>>(v: &mut V, cmd: &'a ast::DefaultSimpleCommand) {
    let ast::SimpleCommand {
        redirects_or_env_vars,
        redirects_or_cmd_words,
    } = cmd;

    let is_assignment = redirects_or_cmd_words.is_empty()
        || redirects_or_env_vars
            .iter()
            .all(|roev| matches!(roev, ast::RedirectOrEnvVar::EnvVar(_, _)));

    if is_assignment {
        for roev in redirects_or_env_vars {
            match roev {
                ast::RedirectOrEnvVar::EnvVar(name, value) => v.visit_var_assignment(name, value),
                ast::RedirectOrEnvVar::Redirect(_) => unreachable!(),
            }
        }
        return;
    }

    for roev in redirects_or_env_vars {
        match roev {
            ast::RedirectOrEnvVar::EnvVar(name, value) => v.visit_env_var(name, value),
            ast::RedirectOrEnvVar::Redirect(redirect) => v.visit_redirect(redirect),
        }
    }

    for rocw in redirects_or_cmd_words {
        match rocw {
            ast::RedirectOrCmdWord::CmdWord(w) => v.visit_top_level_word(w),
            ast::RedirectOrCmdWord::Redirect(redirect) => v.visit_redirect(redirect),
        }
    }
}

pub fn walk_redirect<'a, V: Visitor<'a>>(v: &mut V, redirect: &'a ast::DefaultRedirect) {
    match redirect {
        ast::Redirect::Read(n, w) => v.visit_redirect_read(n, w),
        ast::Redirect::Write(n, w) => v.visit_redirect_write(n, w),
        ast::Redirect::ReadWrite(n, w) => v.visit_redirect_readwrite(n, w),
        ast::Redirect::Append(n, w) => v.visit_redirect_append(n, w),
        ast::Redirect::Clobber(n, w) => v.visit_redirect_clobber(n, w),
        ast::Redirect::Heredoc(n, w) => v.visit_redirect_heredoc(n, w),
        ast::Redirect::DupRead(n, w) => v.visit_redirect_dup_read(n, w),
        ast::Redirect::DupWrite(n, w) => v.visit_redirect_dup_write(n, w),
    }
}

pub fn walk_compound_command<'a, V: Visitor<'a>>(
    v: &mut V,
    kind: &'a ast::DefaultCompoundCommandKind,
    io: &'a [ast::DefaultRedirect],
) {
    v.visit_compound_command_kind(kind);

    for io in io {
        v.visit_redirect(io);
    }
}

pub fn walk_compound_command_kind<'a, V: Visitor<'a>>(
    v: &mut V,
    kind: &'a ast::DefaultCompoundCommandKind,
) {
    match kind {
        ast::CompoundCommandKind::Brace(b) => v.visit_brace_command(b),
        ast::CompoundCommandKind::Subshell(s) => v.visit_subshell(s),
        ast::CompoundCommandKind::While(w) => v.visit_while_loop(w),
        ast::CompoundCommandKind::Until(u) => v.visit_until_loop(u),
        ast::CompoundCommandKind::If {
            conditionals,
            else_branch,
        } => v.visit_if(conditionals, else_branch.as_ref().map(Vec::as_slice)),
        ast::CompoundCommandKind::For { var, words, body } => {
            v.visit_for(var, words.as_ref().map(Vec::as_slice), body)
        }
        ast::CompoundCommandKind::Case { word, arms } => v.visit_case(word, arms),
    }
}

pub fn walk_env_var<'a, V: Visitor<'a>>(
    v: &mut V,
    name: &'a str,
    value: &'a Option<ast::TopLevelWord<String>>,
) {
    let _name = name;
    if let Some(value) = value {
        v.visit_top_level_word(value);
    }
}

pub fn walk_var_assignment<'a, V: Visitor<'a>>(
    v: &mut V,
    name: &'a str,
    value: &'a Option<ast::TopLevelWord<String>>,
) {
    let _name = name;
    if let Some(value) = value {
        v.visit_top_level_word(value);
    }
}

pub fn walk_top_level_word<'a, V: Visitor<'a>>(v: &mut V, word: &'a ast::TopLevelWord<String>) {
    let ast::TopLevelWord(ref inner) = word;
    v.visit_complex_word(inner);
}

pub fn walk_complex_word<'a, V: Visitor<'a>>(v: &mut V, word: &'a ast::DefaultComplexWord) {
    match word {
        ast::ComplexWord::Single(w) => v.visit_word(w),
        ast::ComplexWord::Concat(w) => v.visit_concat_word(w),
    }
}

pub fn walk_word<'a, V: Visitor<'a>>(v: &mut V, word: &'a ast::DefaultWord) {
    match word {
        ast::DefaultWord::Simple(s) => v.visit_simple_word(s),
        ast::DefaultWord::SingleQuoted(s) => v.visit_single_quoted(s),
        ast::DefaultWord::DoubleQuoted(d) => v.visit_double_quoted(d),
    }
}

pub fn walk_concat_word<'a, V: Visitor<'a>>(v: &mut V, words: &'a [ast::DefaultWord]) {
    for w in words {
        v.visit_word(w);
    }
}

pub fn walk_simple_word<'a, V: Visitor<'a>>(v: &mut V, word: &'a ast::DefaultSimpleWord) {
    match word {
        ast::SimpleWord::Literal(l) => v.visit_literal_word(l),
        ast::SimpleWord::Escaped(l) => v.visit_esaped_word(l),
        ast::SimpleWord::Param(p) => v.visit_param(p),
        ast::SimpleWord::Subst(s) => v.visit_param_subst(s),
        ast::SimpleWord::Star => v.visit_star_word(),
        ast::SimpleWord::Question => v.visit_question_word(),
        ast::SimpleWord::SquareOpen => v.visit_square_open_word(),
        ast::SimpleWord::SquareClose => v.visit_square_close_word(),
        ast::SimpleWord::Tilde => v.visit_tilde_word(),
        ast::SimpleWord::Colon => v.visit_colon_word(),
    }
}

pub fn walk_double_quoted<'a, V: Visitor<'a>>(v: &mut V, words: &'a [ast::DefaultSimpleWord]) {
    for w in words {
        v.visit_simple_word(w);
    }
}

pub fn walk_param_subst<'a, V: Visitor<'a>>(
    v: &mut V,
    subst: &'a ast::DefaultParameterSubstitution,
) {
    match subst {
        ast::ParameterSubstitution::Command(cmds) => v.visit_cmd_subst(cmds),
        ast::ParameterSubstitution::Len(param) => v.visit_param_subst_len(param),
        ast::ParameterSubstitution::Arith(arith) => v.visit_param_subst_arith(arith),
        ast::ParameterSubstitution::Default(b, p, s) => v.visit_param_subst_default(*b, p, s),
        ast::ParameterSubstitution::Assign(b, p, s) => v.visit_param_subst_assign(*b, p, s),
        ast::ParameterSubstitution::Error(b, p, s) => v.visit_param_subst_error(*b, p, s),
        ast::ParameterSubstitution::Alternative(b, p, s) => {
            v.visit_param_subst_alternative(*b, p, s)
        }
        ast::ParameterSubstitution::RemoveSmallestSuffix(p, s) => {
            v.visit_param_subst_remove_smallest_suffix(p, s)
        }
        ast::ParameterSubstitution::RemoveLargestSuffix(p, s) => {
            v.visit_param_subst_remove_largest_suffix(p, s)
        }
        ast::ParameterSubstitution::RemoveSmallestPrefix(p, s) => {
            v.visit_param_subst_remove_smallest_prefix(p, s)
        }
        ast::ParameterSubstitution::RemoveLargestPrefix(p, s) => {
            v.visit_param_subst_remove_largest_prefix(p, s)
        }
    }
}

pub fn walk_word_substitution<'a, V: Visitor<'a>>(
    v: &mut V,
    strict: bool,
    param: &'a ast::DefaultParameter,
    subst: &'a Option<ast::TopLevelWord<String>>,
) {
    let _strict = strict;
    v.visit_param(param);

    if let Some(subst) = subst {
        v.visit_top_level_word(subst);
    }
}

pub fn walk_word_substitution_removal<'a, V: Visitor<'a>>(
    v: &mut V,
    param: &'a ast::DefaultParameter,
    subst: &'a Option<ast::TopLevelWord<String>>,
) {
    v.visit_param(param);

    if let Some(subst) = subst {
        v.visit_top_level_word(subst);
    }
}

pub fn walk_top_level_word_array<'a, V: Visitor<'a>>(
    v: &mut V,
    words: &'a [ast::TopLevelWord<String>],
) {
    for word in words {
        walk_top_level_word(v, word);
    }
}

pub fn walk_redirect_read<'a, V: Visitor<'a>>(
    v: &mut V,
    n: &'a Option<u16>,
    file: &'a ast::TopLevelWord<String>,
) {
    let _n = n;
    v.visit_top_level_word(file);
}

pub fn walk_redirect_write<'a, V: Visitor<'a>>(
    v: &mut V,
    n: &'a Option<u16>,
    file: &'a ast::TopLevelWord<String>,
) {
    let _n = n;
    v.visit_top_level_word(file);
}

pub fn walk_redirect_readwrite<'a, V: Visitor<'a>>(
    v: &mut V,
    n: &'a Option<u16>,
    file: &'a ast::TopLevelWord<String>,
) {
    let _n = n;
    v.visit_top_level_word(file);
}

pub fn walk_redirect_append<'a, V: Visitor<'a>>(
    v: &mut V,
    n: &'a Option<u16>,
    file: &'a ast::TopLevelWord<String>,
) {
    let _n = n;
    v.visit_top_level_word(file);
}

pub fn walk_redirect_clobber<'a, V: Visitor<'a>>(
    v: &mut V,
    n: &'a Option<u16>,
    file: &'a ast::TopLevelWord<String>,
) {
    let _n = n;
    v.visit_top_level_word(file);
}

pub fn walk_redirect_heredoc<'a, V: Visitor<'a>>(
    v: &mut V,
    n: &'a Option<u16>,
    file: &'a ast::TopLevelWord<String>,
) {
    let _n = n;
    v.visit_top_level_word(file);
}

pub fn walk_redirect_dup_read<'a, V: Visitor<'a>>(
    v: &mut V,
    n: &'a Option<u16>,
    file: &'a ast::TopLevelWord<String>,
) {
    let _n = n;
    v.visit_top_level_word(file);
}

pub fn walk_redirect_dup_write<'a, V: Visitor<'a>>(
    v: &mut V,
    n: &'a Option<u16>,
    file: &'a ast::TopLevelWord<String>,
) {
    let _n = n;
    v.visit_top_level_word(file);
}
