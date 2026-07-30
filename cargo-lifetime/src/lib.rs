use std::collections::HashMap;

pub const CYAN: &str = "\x1b[1;36m";
pub const YELLOW: &str = "\x1b[1;33m";
pub const RED: &str = "\x1b[1;31m";
pub const GREEN: &str = "\x1b[1;32m";
pub const DIM: &str = "\x1b[2m";
pub const RESET: &str = "\x1b[0m";

const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "continue", "else", "for", "if",
    "in", "let", "loop", "match", "return", "where", "while", "yield",
    "move", "ref", "mut", "unsafe", "impl", "fn", "struct", "enum",
    "trait", "type", "const", "static", "pub", "crate", "self", "super",
    "mod", "use", "extern", "true", "false",
];

#[derive(Clone, Debug)]
pub struct Config {
    extra_safe_fns: Vec<String>,
    extra_pointer_prefixes: Vec<String>,
    extra_pointer_suffixes: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self { extra_safe_fns: Vec::new(), extra_pointer_prefixes: Vec::new(), extra_pointer_suffixes: Vec::new() }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_safe_fn(mut self, name: &str) -> Self {
        self.extra_safe_fns.push(name.to_string());
        self
    }

    pub fn add_pointer_prefix(mut self, prefix: &str) -> Self {
        self.extra_pointer_prefixes.push(prefix.to_string());
        self
    }

    pub fn add_pointer_suffix(mut self, suffix: &str) -> Self {
        self.extra_pointer_suffixes.push(suffix.to_string());
        self
    }

    pub fn is_safe_fn(&self, name: &str) -> bool {
        builtin_safe_fn(name) || self.extra_safe_fns.iter().any(|f| f == name)
    }

    pub fn is_pointer_expr(&self, e: &str) -> bool {
        let e = e.trim();
        builtin_pointer_expr(e)
            || self.extra_pointer_prefixes.iter().any(|p| e.starts_with(p.as_str()))
            || self.extra_pointer_suffixes.iter().any(|s| e.ends_with(s.as_str()))
    }

    pub fn load(path: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("failed to read config '{}': {}", path, e))?;
        let mut config = Self::new();
        for (i, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let colon = line.find(':').ok_or_else(|| {
                format!("{}:{}: expected 'key: value'", path, i + 1)
            })?;
            let key = line[..colon].trim();
            let value = line[colon + 1..].trim();
            match key {
                "safe_fn" => config.extra_safe_fns.push(value.to_string()),
                "prefix" => config.extra_pointer_prefixes.push(value.to_string()),
                "suffix" => config.extra_pointer_suffixes.push(value.to_string()),
                _ => return Err(format!("{}:{}: unknown key '{}'", path, i + 1, key)),
            }
        }
        Ok(config)
    }
}

fn builtin_safe_fn(name: &str) -> bool {
    matches!(
        name,
        "print" | "println" | "eprint" | "eprintln"
            | "write" | "writeln"
            | "format" | "format_args"
            | "assert" | "assert_eq" | "assert_ne"
            | "debug_assert" | "debug_assert_eq" | "debug_assert_ne"
            | "panic" | "unreachable" | "unimplemented" | "todo"
            | "dbg" | "stringify" | "concat" | "include_str" | "include_bytes"
            | "Vec::new" | "vec" | "vec!"
            | "Box::new" | "Box::pin"
            | "Rc::new" | "Rc::pin"
            | "Arc::new" | "Arc::pin"
            | "String::new" | "String::from" | "String::with_capacity"
            | "format!"
            | "Some" | "Ok" | "Err" | "None"
            | "std::mem::drop" | "mem::drop" | "drop"
            | "std::mem::forget" | "mem::forget" | "forget"
            | "std::mem::replace" | "mem::replace"
            | "std::mem::take" | "mem::take"
            | "std::mem::swap" | "mem::swap"
            | "core::mem::drop" | "core::mem::forget"
            | "std::ptr::read" | "ptr::read"
            | "std::ptr::write" | "ptr::write"
            | "std::ptr::replace" | "ptr::replace"
            | "std::ptr::drop_in_place" | "ptr::drop_in_place"
            | "std::sync::Arc::clone" | "Arc::clone"
            | "std::rc::Rc::clone" | "Rc::clone"
            | "clone"
            | "as_ref" | "as_mut"
            | "into" | "from"
            | "as_ptr" | "as_mut_ptr"
            | "len" | "is_empty" | "capacity"
            | "unwrap" | "expect" | "ok" | "err"
            | "map" | "and_then" | "or_else"
            | "iter" | "into_iter" | "iter_mut"
    )
}

fn builtin_pointer_expr(e: &str) -> bool {
    let starts = e.starts_with('&')
        || e.starts_with("&raw ")
        || e.starts_with("&raw mut ")
        || e.starts_with("Box::into_raw(")
        || e.starts_with("std::boxed::Box::into_raw(")
        || e.starts_with("Rc::as_ptr(")
        || e.starts_with("std::rc::Rc::as_ptr(")
        || e.starts_with("Arc::as_ptr(")
        || e.starts_with("std::sync::Arc::as_ptr(")
        || e.starts_with("NonNull::new(")
        || e.starts_with("std::ptr::NonNull::new(")
        || e.starts_with("NonNull::new_unchecked(")
        || e.starts_with("std::ptr::NonNull::new_unchecked(")
        || e.starts_with("Pin::new(")
        || e.starts_with("std::pin::Pin::new(")
        || e.starts_with("Pin::new_unchecked(")
        || e.starts_with("std::pin::Pin::new_unchecked(")
        || e.starts_with("ManuallyDrop::new(")
        || e.starts_with("std::mem::ManuallyDrop::new(")
        || e.starts_with("Cell::new(")
        || e.starts_with("std::cell::Cell::new(")
        || e.starts_with("RefCell::new(")
        || e.starts_with("std::cell::RefCell::new(")
        || e.starts_with("Mutex::new(")
        || e.starts_with("std::sync::Mutex::new(")
        || e.starts_with("RwLock::new(")
        || e.starts_with("std::sync::RwLock::new(")
        || e.starts_with("UnsafeCell::new(")
        || e.starts_with("std::cell::UnsafeCell::new(")
        || e.starts_with("Cow::Borrowed(")
        || e.starts_with("std::borrow::Cow::Borrowed(");
    let ends = e.ends_with(".as_ptr()")
        || e.ends_with(".as_mut_ptr()")
        || e.ends_with(".as_ref()")
        || e.ends_with(".as_mut()")
        || e.ends_with(".as_bytes()")
        || e.ends_with(".as_bytes_mut()")
        || e.ends_with(".as_slice()")
        || e.ends_with(".as_mut_slice()")
        || e.ends_with(".as_str()")
        || e.ends_with(".as_mut_str()")
        || e.ends_with(".as_deref()")
        || e.ends_with(".as_deref_mut()")
        || e.ends_with(".borrow()")
        || e.ends_with(".borrow_mut()")
        || e.ends_with(".borrow_ref()")
        || e.ends_with(".borrow_ref_mut()")
        || e.ends_with(".pin()")
        || e.ends_with(".pin_mut()");
    starts || ends
}

fn find_matching_paren(s: &str, open_pos: usize) -> Option<usize> {
    if !s.is_char_boundary(open_pos) { return None; }
    if s[open_pos..].chars().next()? != '(' { return None; }
    let mut depth: isize = 1;
    let mut chars = s[open_pos + 1..].char_indices();
    while let Some((offset, ch)) = chars.next() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(open_pos + 1 + offset);
                }
            }
            '"' => {
                while let Some((_, sc)) = chars.next() {
                    if sc == '\\' { chars.next(); }
                    else if sc == '"' { break; }
                }
            }
            '/' => {
                if let Some(&next) = s.as_bytes().get(open_pos + 1 + offset + 1) {
                    if next == b'/' {
                        while let Some((_, sc)) = chars.next() {
                            if sc == '\n' { break; }
                        }
                    } else if next == b'*' {
                        let mut prev_star = false;
                        while let Some((_, sc)) = chars.next() {
                            if sc == '*' { prev_star = true; }
                            else if sc == '/' && prev_star { break; }
                            else { prev_star = false; }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    None
}

fn smart_split(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut depth_paren: isize = 0;
    let mut depth_bracket: isize = 0;
    let mut depth_brace: isize = 0;
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '"' => {
                i += 1;
                while i < chars.len() {
                    if chars[i] == '\\' { i += 2; continue; }
                    if chars[i] == '"' { break; }
                    i += 1;
                }
            }
            '/' if i + 1 < chars.len() => {
                if chars[i + 1] == '/' {
                    let seg: String = s[start..i].trim().to_string();
                    if !seg.is_empty() { parts.push(seg); }
                    return parts;
                }
                if chars[i + 1] == '*' {
                    i += 2;
                    loop {
                        if i + 1 >= chars.len() { break; }
                        if chars[i] == '*' && chars[i + 1] == '/' { i += 1; break; }
                        i += 1;
                    }
                }
            }
            '(' => depth_paren += 1,
            ')' => depth_paren -= 1,
            '[' => depth_bracket += 1,
            ']' => depth_bracket -= 1,
            '{' => depth_brace += 1,
            '}' => depth_brace -= 1,
            ';' if depth_paren == 0 && depth_bracket == 0 && depth_brace == 0 => {
                let seg: String = s[start..i].trim().to_string();
                if !seg.is_empty() { parts.push(seg); }
                start = i + 1;
            }
            _ => {}
        }
        i += 1;
    }

    let seg: String = s[start..].trim().to_string();
    if !seg.is_empty() { parts.push(seg); }
    parts
}

pub fn check_source(src: &str) -> Vec<(usize, String)> {
    check_source_with_config(src, &Config::default())
}

pub fn check_source_with_config(src: &str, config: &Config) -> Vec<(usize, String)> {
    let mut sc = Scanner::new(config.clone());
    let lines = join_lt_lines(src);
    for &(line_num, ref raw) in &lines {
        let s = raw.trim();
        if s.is_empty() { continue; }

        let s = if let Some(cfg_line) = strip_cfg_test(s) { cfg_line } else { s };
        if s.is_empty() { continue; }

        let mut seg_start = 0;
        let mut in_str = false;
        let mut in_block_comment: isize = 0;
        for (j, ch) in s.char_indices() {
            if ch == '"' { in_str = !in_str; continue; }
            if in_str { continue; }

            if ch == '/' {
                if s[j..].starts_with("/*") { in_block_comment += 1; continue; }
                if s[j..].starts_with("*/") && in_block_comment > 0 {
                    in_block_comment -= 1;
                    seg_start = j + 2;
                    continue;
                }
            }
            if in_block_comment > 0 { continue; }

            if s[j..].starts_with("//") { break; }

            if ch == '}' || ch == '{' {
                let seg = s[seg_start..j].trim();
                let seg = strip_line_comment(seg);
                if !seg.is_empty() {
                    for sub in smart_split(seg) {
                        let sub = sub.trim().to_string();
                        if !sub.is_empty() {
                            sc.analyze_segment(&sub, line_num);
                        }
                    }
                }
                if ch == '}' {
                    sc.exit_scope(line_num);
                    sc.depth = sc.depth.saturating_sub(1);
                } else {
                    sc.depth += 1;
                }
                seg_start = j + 1;
            }
        }
        let seg = s[seg_start..].trim();
        let seg = strip_line_comment(seg);
        if !seg.is_empty() {
            for sub in smart_split(seg) {
                let sub = sub.trim().to_string();
                if !sub.is_empty() {
                    sc.analyze_segment(&sub, line_num);
                }
            }
        }
    }

    std::mem::take(&mut sc.errors)
}

fn strip_cfg_test<'a>(s: &'a str) -> Option<&'a str> {
    if !s.starts_with("#[cfg(") { return None; }
    let end = find_matching_paren(s, 5)?;
    let inner = &s[6..end];
    if cfg_predicate_has_test(inner) {
        if !s[end + 1..].starts_with(']') { return None; }
        let rest = s[end + 2..].trim();
        if rest.is_empty() { Some("") } else { Some(rest) }
    } else {
        None
    }
}

fn cfg_predicate_has_test(pred: &str) -> bool {
    let pred = pred.trim();
    if pred == "test" { return true; }
    if pred.starts_with("not(") { return false; }
    if pred.starts_with("all(") || pred.starts_with("any(") {
        let paren = pred.find('(').unwrap_or(0);
        if let Some(end) = find_matching_paren(pred, paren) {
            let body = &pred[paren + 1..end];
            split_top_level_commas(body).iter().any(|a| cfg_predicate_has_test(a.trim()))
        } else {
            false
        }
    } else {
        pred.contains("test")
    }
}

fn strip_line_comment<'a>(s: &'a str) -> &'a str {
    if let Some(pos) = s.find("//") {
        let before = &s[..pos].trim_end();
        if before.is_empty() { "" } else { before }
    } else {
        s
    }
}

fn has_unclosed_lt(s: &str) -> bool {
    let mut in_str = false;
    let mut i = 0;
    let chars: Vec<char> = s.chars().collect();
    while i < chars.len() {
        if chars[i] == '"' { in_str = !in_str; i += 1; continue; }
        if in_str { i += 1; continue; }
        if i + 4 <= chars.len() && chars[i] == 'l' && chars[i+1] == 't' && chars[i+2] == '!' && chars[i+3] == '(' {
            match find_matching_paren(s, i + 3) {
                Some(pos) => { i = pos + 1; }
                None => return true,
            }
            continue;
        }
        i += 1;
    }
    false
}

fn join_lt_lines(src: &str) -> Vec<(usize, String)> {
    let mut result = Vec::new();
    let mut pending: Option<(usize, String)> = None;

    for (i, raw) in src.lines().enumerate() {
        let line_num = i + 1;
        let s = raw.to_string();

        if let Some((first_line, mut acc)) = pending.take() {
            acc.push('\n');
            acc.push_str(&s);
            if !has_unclosed_lt(&acc) {
                result.push((first_line, acc));
            } else {
                pending = Some((first_line, acc));
            }
            continue;
        }

        if has_unclosed_lt(&s) {
            pending = Some((line_num, s));
        } else {
            result.push((line_num, s));
        }
    }

    if let Some(p) = pending { result.push(p); }
    result
}

pub fn pline(path: &str, line: usize, msg: &str) {
    println!("{CYAN}{path}:{line}:{RESET} {}", colorize(msg));
}

pub fn adjust_line_in_msg(msg: &str, n: usize) -> String {
    let mut r = String::new();
    let mut rest = msg;
    while let Some(start) = rest.find("(declared at line ") {
        r.push_str(&rest[..start]);
        r.push_str("(declared at line ");
        let off = start + "(declared at line ".len();
        let end = rest[off..].find(')').map(|e| off + e).unwrap_or(rest.len());
        let raw: usize = rest[off..end].parse().unwrap_or(1);
        r.push_str(&raw.saturating_sub(n).to_string());
        r.push(')');
        rest = &rest[end + 1..];
    }
    r.push_str(rest);
    r
}

pub fn print_file_errors(path: &str, errors: &[(usize, String)]) {
    for (line, msg) in errors {
        pline(path, *line, msg);
    }
}

fn colorize(msg: &str) -> String {
    let mut r = String::new();
    let mut i = 0;
    let b = msg.as_bytes();
    while i < b.len() {
        let ch = match msg[i..].chars().next() {
            Some(c) => c,
            None => break,
        };
        if ch == '[' {
            if let Some(end) = msg[i..].find(']') {
                r.push_str(YELLOW);
                r.push_str(&msg[i..=i + end]);
                r.push_str(RESET);
                i += end + 1;
                continue;
            }
        }
        if ch == '`' {
            if let Some(end) = msg[i + 1..].find('`') {
                r.push('`');
                r.push_str(CYAN);
                r.push_str(&msg[i + 1..=i + end]);
                r.push_str(RESET);
                r.push('`');
                i += end + 2;
                continue;
            }
        }
        if msg[i..].starts_with("(declared at line ") {
            if let Some(end) = msg[i..].find(')') {
                r.push_str(DIM);
                r.push_str(&msg[i..=i + end]);
                r.push_str(RESET);
                i += end + 1;
                continue;
            }
        }
        r.push(ch);
        i += ch.len_utf8();
    }
    r
}

struct Owner {
    var: String,
    label: String,
    line: usize,
    depth: usize,
}

struct Borrow {
    var: String,
    label: String,
    line: usize,
    depth: usize,
}

struct Scanner {
    config: Config,
    depth: usize,
    var_depth: HashMap<String, usize>,
    owners: Vec<Owner>,
    borrows: Vec<Borrow>,
    errors: Vec<(usize, String)>,
}

impl Scanner {
    fn new(config: Config) -> Self {
        Scanner {
            config,
            depth: 0,
            var_depth: HashMap::new(),
            owners: Vec::new(),
            borrows: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn analyze_segment(&mut self, code: &str, line: usize) {
        if let Some((var, expr, label)) = parse_lt_let(code) {
            self.remove_at_depth(&var);
            self.var_depth.insert(var.clone(), self.depth);
            if self.config.is_pointer_expr(&expr) {
                self.borrows.push(Borrow { var, label, line, depth: self.depth });
            } else {
                self.owners.push(Owner { var, label, line, depth: self.depth });
            }
            return;
        }

        if let Some(vars) = parse_destructure(code) {
            for var in &vars {
                self.remove_at_depth(var);
                self.var_depth.insert(var.clone(), self.depth);
            }
            return;
        }

        if let Some(var) = parse_let(code) {
            self.remove_at_depth(&var);
            self.var_depth.insert(var, self.depth);
            return;
        }

        if let Some(var) = parse_for(code) {
            self.remove_at_depth(&var);
            self.var_depth.insert(var, self.depth);
            return;
        }

        if let Some(vars) = parse_destructure_assign(code) {
            for var in &vars {
                self.owners.retain(|o| o.var != *var);
                self.borrows.retain(|b| b.var != *var);
            }
            return;
        }

        if let Some((var, _expr, label)) = parse_lt_assign(code) {
            let decl_d = self.var_depth.get(&var).copied().unwrap_or(self.depth);
            let eff_d = decl_d.min(self.depth);
            self.owners.retain(|o| o.var != var);
            self.borrows.retain(|b| b.var != var);
            self.borrows.push(Borrow { var, label, line, depth: eff_d });
            return;
        }

        if let Some(var) = parse_drop(code) {
            self.check_drop(&var, line);
            return;
        }

        self.check_owner_used(code, line);
    }

    fn remove_at_depth(&mut self, var: &str) {
        self.owners.retain(|o| !(o.var == var && o.depth == self.depth));
        self.borrows.retain(|b| !(b.var == var && b.depth == self.depth));
    }

    fn exit_scope(&mut self, line: usize) {
        let d = self.depth;

        let owner_idxs: Vec<usize> = self
            .owners
            .iter()
            .enumerate()
            .filter(|(_, o)| o.depth == d)
            .map(|(i, _)| i)
            .collect();

        for &i in &owner_idxs {
            let o = &self.owners[i];
            for b in &self.borrows {
                if b.label != o.label { continue; }
                let alive = b.depth < d || (b.depth == d && b.line < o.line);
                if alive {
                    self.errors.push((
                        line,
                        format!(
                            "[{}] `{}` (declared at line {}) dropped at end of scope but `{}` (declared at line {}) still has active borrow(s)",
                            o.label, o.var, o.line, b.var, b.line
                        ),
                    ));
                }
            }
        }

        self.owners.retain(|o| o.depth != d);
        self.borrows.retain(|b| b.depth != d);
    }

    fn check_owner_used(&mut self, code: &str, line: usize) {
        let c = code.trim();
        let lparen = match c.find('(') {
            Some(p) => p,
            None => return,
        };
        let fn_name = c[..lparen].trim();
        if fn_name.is_empty() || fn_name.contains(' ') || fn_name.contains('=') { return; }
        if self.config.is_safe_fn(fn_name) { return; }
        if KEYWORDS.contains(&fn_name) { return; }

        let args = match extract_paren_body(&c[lparen..]) {
            Some(v) => v,
            None => return,
        };
        for arg in split_top_level_commas(args) {
            let arg = arg.trim();
            if !is_ident(arg) { continue; }
            if let Some(o) = self.owners.iter().find(|o| o.var == arg) {
                for b in &self.borrows {
                    if b.label == o.label {
                        self.errors.push((
                            line,
                            format!(
                                "[{}] `{}` passed to function while `{}` (declared at line {}) has active borrow(s)",
                                o.label, o.var, b.var, b.line
                            ),
                        ));
                    }
                }
            }
        }
    }

    fn check_drop(&mut self, var: &str, line: usize) {
        for o in &self.owners {
            if o.var != var { continue; }
            for b in &self.borrows {
                if b.label == o.label && b.line < line {
                    self.errors.push((
                        line,
                        format!(
                            "[{}] `drop({})` called but `{}` (declared at line {}) has active borrow(s)",
                            o.label, var, b.var, b.line
                        ),
                    ));
                }
            }
        }
    }
}

fn extract_paren_body(s: &str) -> Option<&str> {
    let s = s.trim();
    if !s.starts_with('(') { return None; }
    let end = find_matching_paren(s, 0)?;
    Some(&s[1..end])
}

fn split_top_level_commas(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut depth_paren: isize = 0;
    let mut depth_bracket: isize = 0;
    let mut depth_brace: isize = 0;
    let mut in_str = false;
    for (i, ch) in s.char_indices() {
        match ch {
            '"' => in_str = !in_str,
            _ if in_str => {}
            '(' => depth_paren += 1,
            ')' => depth_paren -= 1,
            '[' => depth_bracket += 1,
            ']' => depth_bracket -= 1,
            '{' => depth_brace += 1,
            '}' => depth_brace -= 1,
            ',' if depth_paren == 0 && depth_bracket == 0 && depth_brace == 0 => {
                parts.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(&s[start..]);
    parts
}

fn parse_lt_let(code: &str) -> Option<(String, String, String)> {
    let c = code.trim();
    if !c.starts_with("let ") { return None; }
    let rest = &c[4..];
    let mut word = rest.trim_start();
    loop {
        let end = word.find(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_').unwrap_or(word.len());
        let token = &word[..end];
        if token == "mut" || token == "ref" {
            word = word[end..].trim_start();
            continue;
        }
        if token.is_empty() || !is_ident(token) { return None; }
        let eq = word[end..].find('=')?;
        let var = token.to_string();
        let after = word[end + eq + 1..].trim();
        return extract_lt_expr_label(after).map(|(expr, label)| (var, expr, label));
    }
}

fn parse_let(code: &str) -> Option<String> {
    let c = code.trim();
    if !c.starts_with("let ") { return None; }
    let rest = &c[4..];
    let mut word = rest.trim_start();
    loop {
        let end = word.find(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_').unwrap_or(word.len());
        let token = &word[..end];
        if token == "mut" || token == "ref" {
            word = word[end..].trim_start();
            continue;
        }
        if token.is_empty() || !is_ident(token) { return None; }
        return Some(token.to_string());
    }
}

fn parse_for(code: &str) -> Option<String> {
    let c = code.trim();
    if !c.starts_with("for ") { return None; }
    let mut rest = &c[4..];
    loop {
        let trimmed = rest.trim_start();
        if trimmed.starts_with("mut ") { rest = &trimmed[4..]; continue; }
        if trimmed.starts_with('&') { rest = &trimmed[1..]; continue; }
        rest = trimmed;
        break;
    }
    if rest.is_empty() { return None; }
    let next = rest.split(|ch: char| ch.is_whitespace() || ch == '(').next()?;
    if is_ident(next) { Some(next.to_string()) } else { None }
}

fn collect_identifiers_from_pattern(s: &str) -> Vec<String> {
    let mut vars = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '"' => {
                i += 1;
                while i < chars.len() && chars[i] != '"' {
                    if chars[i] == '\\' { i += 1; }
                    i += 1;
                }
                if i < chars.len() { i += 1; }
            }
            '(' | '[' => {
                let close = if chars[i] == '(' { ')' } else { ']' };
                let mut depth: isize = 1;
                let mut j = i + 1;
                while j < chars.len() && depth > 0 {
                    if chars[j] == chars[i] { depth += 1; }
                    else if chars[j] == close { depth -= 1; }
                    j += 1;
                }
                let inner: String = chars[i + 1..j - 1].iter().collect();
                for part in split_top_level_commas(&inner) {
                    vars.extend(collect_identifiers_from_pattern(part.trim()));
                }
                i = j;
            }
            '{' => {
                let mut depth: isize = 1;
                let mut j = i + 1;
                while j < chars.len() && depth > 0 {
                    if chars[j] == '{' { depth += 1; }
                    else if chars[j] == '}' { depth -= 1; }
                    j += 1;
                }
                let inner: String = chars[i + 1..j - 1].iter().collect();
                for field in inner.split(',') {
                    let field = field.trim();
                    if let Some(eq_pos) = field.find(':') {
                        let val = field[eq_pos + 1..].trim();
                        if val.starts_with('{') {
                            vars.extend(collect_identifiers_from_pattern(val));
                        } else if is_ident(val) {
                            vars.push(val.to_string());
                        }
                    } else if !field.is_empty() && is_ident(field) {
                        vars.push(field.to_string());
                    }
                }
                i = j;
            }
            '&' | '*' => { i += 1; }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut end = i + 1;
                while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
                    end += 1;
                }
                let name: String = chars[i..end].iter().collect();
                if end < chars.len() && chars[end] == '@' {
                    i = end + 1;
                    continue;
                }
                if end < chars.len() && chars[end] == '!' { i = end + 1; continue; }
                if name != "_" && is_ident(&name) {
                    vars.push(name);
                }
                i = end;
            }
            _ => { i += 1; }
        }
    }
    vars
}

fn parse_destructure(code: &str) -> Option<Vec<String>> {
    let c = code.trim();
    if !c.starts_with("let ") { return None; }
    let mut rest = &c[4..];
    loop {
        let trimmed = rest.trim_start();
        if trimmed.starts_with("mut ") { rest = &trimmed[4..]; continue; }
        if trimmed.starts_with("ref ") { rest = &trimmed[4..]; continue; }
        if trimmed.starts_with('&') { rest = &trimmed[1..]; continue; }
        rest = trimmed;
        break;
    }
    if rest.is_empty() { return None; }

    let first = rest.chars().next()?;
    if first == '(' || first == '[' || first == '{' || rest.starts_with("Some(") || rest.starts_with("Ok(") || rest.starts_with("Err(") {
        let vars = collect_identifiers_from_pattern(rest);
        if vars.is_empty() { None } else { Some(vars) }
    } else {
        None
    }
}

fn parse_destructure_assign(code: &str) -> Option<Vec<String>> {
    let c = code.trim();
    if c.starts_with("let ") { return None; }
    if c.starts_with("for ") { return None; }
    let eq = find_non_relational_eq(c)?;
    let pattern = c[..eq].trim();
    if pattern.is_empty() { return None; }
    if !pattern.contains('{') && !pattern.contains('(') && !pattern.contains('[') {
        return None;
    }
    let mut vars = collect_identifiers_from_pattern(pattern);
    // Strip leading type/variant identifier before { or (
    if let Some(brace_pos) = pattern.find(|c: char| c == '{' || c == '(' || c == '[') {
        let before = pattern[..brace_pos].trim();
        if !before.is_empty() && !vars.is_empty() && vars[0] == before {
            vars.remove(0);
        }
    }
    if vars.is_empty() { None } else { Some(vars) }
}

fn parse_lt_assign(code: &str) -> Option<(String, String, String)> {
    let c = code.trim();
    if c.starts_with("let ") { return None; }
    let eq = find_non_relational_eq(c)?;
    let var = c[..eq].trim();
    if !is_ident(var) { return None; }
    let after = c[eq + 1..].trim();
    extract_lt_expr_label(after).map(|(expr, label)| (var.to_string(), expr, label))
}

fn find_non_relational_eq(s: &str) -> Option<usize> {
    let mut in_str = false;
    for (i, ch) in s.char_indices() {
        if ch == '"' { in_str = !in_str; }
        else if !in_str && ch == '=' {
            if i > 0 {
                let prev = s.as_bytes()[i - 1];
                if prev == b'!' || prev == b'<' || prev == b'>' || prev == b'=' { continue; }
            }
            if s.as_bytes().get(i + 1) == Some(&b'=') { continue; }
            return Some(i);
        }
    }
    None
}

fn extract_lt_expr_label(s: &str) -> Option<(String, String)> {
    if !s.starts_with("lt!(") { return None; }
    let end = find_matching_paren(s, 3)?;
    let body = s[4..end].trim();
    let last_comma = find_last_comma(body)?;
    let expr = body[..last_comma].trim().to_string();
    let label_raw = body[last_comma + 1..].trim();
    if label_raw.len() < 2 || !label_raw.starts_with('"') || !label_raw.ends_with('"') { return None; }
    let label = label_raw[1..label_raw.len() - 1].to_string();
    Some((expr, label))
}

fn find_last_comma(s: &str) -> Option<usize> {
    let mut in_str = false;
    let mut nest: isize = 0;
    let mut last = None;
    for (i, ch) in s.char_indices() {
        if ch == '"' { in_str = !in_str; }
        else if !in_str {
            match ch {
                '(' | '[' | '{' => nest += 1,
                ')' | ']' | '}' => nest -= 1,
                ',' if nest == 0 => last = Some(i),
                _ => {}
            }
        }
    }
    last
}

fn parse_drop(code: &str) -> Option<String> {
    let c = code.trim();
    if !c.starts_with("drop(") { return None; }
    let end = find_matching_paren(c, 4)?;
    let body = c[5..end].trim();
    if body.is_empty() { return None; }
    Some(body.to_string())
}

pub fn is_ident(s: &str) -> bool {
    if s.is_empty() { return false; }
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    if !first.is_ascii_alphabetic() && first != '_' { return false; }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn strip_config(args: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--config" {
            i += 2;
        } else {
            out.push(args[i].clone());
            i += 1;
        }
    }
    out
}

pub fn load_config(args: &[String]) -> Result<Config, String> {
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--config" {
            if let Some(path) = args.get(i + 1) {
                return Config::load(path);
            } else {
                return Err("--config requires a path argument".to_string());
            }
        }
        i += 1;
    }
    Ok(Config::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_config_default() {
        let c = Config::default();
        assert!(!c.is_safe_fn("my_fn"));
        assert!(!c.is_pointer_expr("MyPtr::new(x)"));
    }

    #[test]
    fn test_config_add_safe_fn() {
        let c = Config::new().add_safe_fn("my_fn");
        assert!(c.is_safe_fn("my_fn"));
        assert!(!c.is_safe_fn("other_fn"));
    }

    #[test]
    fn test_config_add_pointer_prefix() {
        let c = Config::new().add_pointer_prefix("MyPtr::new(");
        assert!(c.is_pointer_expr("MyPtr::new(x)"));
        assert!(!c.is_pointer_expr("Other::new(x)"));
    }

    #[test]
    fn test_config_add_pointer_suffix() {
        let c = Config::new().add_pointer_suffix(".raw()");
        assert!(c.is_pointer_expr("x.raw()"));
        assert!(!c.is_pointer_expr("x.other()"));
    }

    #[test]
    fn test_config_load_valid() {
        let dir = std::env::temp_dir();
        let path = dir.join("_test_lifetime_config_valid.toml");
        let mut f = std::fs::File::create(&path).unwrap();
        write!(f, "safe_fn: custom_fn\nprefix: Ptr::new(\nsuffix: .leak()\n").unwrap();
        let c = Config::load(path.to_str().unwrap()).unwrap();
        assert!(c.is_safe_fn("custom_fn"));
        assert!(c.is_pointer_expr("Ptr::new(x)"));
        assert!(c.is_pointer_expr("x.leak()"));
        assert!(!c.is_safe_fn("other_fn"));
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_config_load_unknown_key() {
        let dir = std::env::temp_dir();
        let path = dir.join("_test_lifetime_config_bad_key.toml");
        let mut f = std::fs::File::create(&path).unwrap();
        write!(f, "unknown: value\n").unwrap();
        let result = Config::load(path.to_str().unwrap());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unknown key"));
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_config_load_missing_colon() {
        let dir = std::env::temp_dir();
        let path = dir.join("_test_lifetime_config_no_colon.toml");
        let mut f = std::fs::File::create(&path).unwrap();
        write!(f, "bad line\n").unwrap();
        let result = Config::load(path.to_str().unwrap());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expected 'key: value'"));
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_config_load_comments_and_blanks() {
        let dir = std::env::temp_dir();
        let path = dir.join("_test_lifetime_config_comments.toml");
        let mut f = std::fs::File::create(&path).unwrap();
        write!(f, "# comment\n\nsafe_fn: fn1\n# another\nsafe_fn: fn2\n").unwrap();
        let c = Config::load(path.to_str().unwrap()).unwrap();
        assert!(c.is_safe_fn("fn1"));
        assert!(c.is_safe_fn("fn2"));
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_builtin_safe_fn() {
        assert!(builtin_safe_fn("println"));
        assert!(builtin_safe_fn("drop"));
        assert!(builtin_safe_fn("clone"));
    }

    #[test]
    fn test_builtin_pointer_expr() {
        assert!(builtin_pointer_expr("&val"));
        assert!(builtin_pointer_expr("val.as_ptr()"));
        assert!(builtin_pointer_expr("Box::into_raw(box)"));
    }

    #[test]
    fn test_custom_safe_fn_suppresses_error() {
        let src = "let val = lt!(vec![1, 2, 3], \"l\");\nlet p = lt!(val.as_ptr(), \"l\");\nmy_fn(val);\n";
        let default_config = Config::default();
        let custom_config = Config::new().add_safe_fn("my_fn");
        let default_errors = check_source_with_config(src, &default_config);
        let custom_errors = check_source_with_config(src, &custom_config);
        assert!(!default_errors.is_empty(), "my_fn should be flagged with default config");
        assert!(custom_errors.is_empty(), "my_fn suppressed by custom safe_fn");
    }

    #[test]
    fn test_custom_prefix_triggers_borrow_tracking() {
        let src = "let val = lt!(vec![1, 2, 3], \"l\");\nlet p = lt!(MyPtr::new(&val), \"l\");\ndrop(val);\n";
        let default_config = Config::default();
        let custom_config = Config::new().add_pointer_prefix("MyPtr::new(");
        let default_errors = check_source_with_config(src, &default_config);
        let custom_errors = check_source_with_config(src, &custom_config);
        assert!(default_errors.is_empty(), "MyPtr not recognized with default config");
        assert!(!custom_errors.is_empty(), "MyPtr recognized as borrow with custom prefix");
    }

    #[test]
    fn test_custom_suffix_triggers_borrow_tracking() {
        let src = "let val = lt!(vec![1, 2, 3], \"l\");\nlet p = lt!(val.custom_ref(), \"l\");\ndrop(val);\n";
        let default_config = Config::default();
        let custom_config = Config::new().add_pointer_suffix(".custom_ref()");
        let default_errors = check_source_with_config(src, &default_config);
        let custom_errors = check_source_with_config(src, &custom_config);
        assert!(default_errors.is_empty(), "custom_ref not recognized with default config");
        assert!(!custom_errors.is_empty(), "custom_ref recognized as borrow with custom suffix");
    }

    // --- smart_split ---

    #[test]
    fn test_smart_split_semicolons() {
        let parts = smart_split("a; b; c");
        assert_eq!(parts, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_smart_split_string_preserves_semicolon() {
        let parts = smart_split(r#"println!("hello; world"); next"#);
        assert_eq!(parts, vec![r#"println!("hello; world")"#, "next"]);
    }

    #[test]
    fn test_smart_split_string_protects_semicolon() {
        let parts = smart_split(r#"a; "b;c"; d"#);
        assert_eq!(parts, vec!["a", r#""b;c""#, "d"]);
    }

    #[test]
    fn test_smart_split_escaped_quote() {
        let parts = smart_split(r#"a; "foo\"bar"; b"#);
        assert_eq!(parts, vec!["a", r#""foo\"bar""#, "b"]);
    }

    // --- find_matching_paren ---

    #[test]
    fn test_find_matching_paren_basic() {
        assert_eq!(find_matching_paren("(a, b)", 0), Some(5));
    }

    #[test]
    fn test_find_matching_paren_nested() {
        assert_eq!(find_matching_paren("(foo(bar))", 0), Some(9));
    }

    #[test]
    fn test_find_matching_paren_string_skips_paren() {
        assert_eq!(find_matching_paren(r#"("foo)", bar)"#, 0), Some(12));
    }

    #[test]
    fn test_find_matching_paren_comment_skips_paren() {
        assert_eq!(find_matching_paren("(/* ) */ )", 0), Some(9));
    }

    // --- parse_drop ---

    #[test]
    fn test_parse_drop_simple() {
        assert_eq!(parse_drop("drop(val)"), Some("val".to_string()));
    }

    #[test]
    fn test_parse_drop_nested() {
        assert_eq!(parse_drop("drop(Box::into_raw(box))"), Some("Box::into_raw(box)".to_string()));
    }

    #[test]
    fn test_parse_drop_no_drop() {
        assert_eq!(parse_drop("observe(val)"), None);
    }

    // --- extract_paren_body ---

    #[test]
    fn test_extract_paren_body_basic() {
        assert_eq!(extract_paren_body("(a, b)"), Some("a, b"));
    }

    #[test]
    fn test_extract_paren_body_nested() {
        assert_eq!(extract_paren_body("(foo(bar), baz)"), Some("foo(bar), baz"));
    }

    #[test]
    fn test_extract_paren_body_with_string() {
        assert_eq!(extract_paren_body(r#"("foo)", bar)"#), Some(r#""foo)", bar"#));
    }

    // --- extract_lt_expr_label ---

    #[test]
    fn test_extract_lt_expr_label_simple() {
        assert_eq!(extract_lt_expr_label(r#"lt!(val.as_ptr(), "l")"#),
                   Some(("val.as_ptr()".to_string(), "l".to_string())));
    }

    #[test]
    fn test_extract_lt_expr_label_nested_expr() {
        assert_eq!(extract_lt_expr_label(r#"lt!(MyBox::new(&val), "l")"#),
                   Some(("MyBox::new(&val)".to_string(), "l".to_string())));
    }

    #[test]
    fn test_extract_lt_expr_label_paren_in_string() {
        assert_eq!(extract_lt_expr_label(r#"lt!(concat!("foo)"), "l")"#),
                   Some(("concat!(\"foo)\")".to_string(), "l".to_string())));
    }

    #[test]
    fn test_extract_lt_expr_label_no_label() {
        assert_eq!(extract_lt_expr_label("lt!(val)"), None);
    }

    // --- find_last_comma ---

    #[test]
    fn test_find_last_comma_basic() {
        assert_eq!(find_last_comma("a, b"), Some(1));
    }

    #[test]
    fn test_find_last_comma_skips_nested() {
        assert_eq!(find_last_comma("foo(bar), baz"), Some(8));
    }

    #[test]
    fn test_find_last_comma_skips_string() {
        let s = r#"foo, "b,ar", baz"#;
        assert_eq!(find_last_comma(s), Some(11));
    }

    // --- cfg_predicate_has_test ---

    #[test]
    fn test_cfg_predicate_test_exact() {
        assert!(cfg_predicate_has_test("test"));
    }

    #[test]
    fn test_cfg_predicate_not_test() {
        assert!(!cfg_predicate_has_test("not(test)"));
    }

    #[test]
    fn test_cfg_predicate_all_test() {
        assert!(cfg_predicate_has_test("all(target_os = \"linux\", test)"));
    }

    #[test]
    fn test_cfg_predicate_any_test() {
        assert!(cfg_predicate_has_test("any(test, target_os = \"windows\")"));
    }

    // --- strip_cfg_test ---

    #[test]
    fn test_strip_cfg_test_simple() {
        assert_eq!(strip_cfg_test("#[cfg(test)] fn f() {}"), Some("fn f() {}"));
    }

    #[test]
    fn test_strip_cfg_test_not() {
        assert_eq!(strip_cfg_test("#[cfg(not(test))] fn f() {}"), None);
    }

    #[test]
    fn test_strip_cfg_test_all() {
        assert_eq!(strip_cfg_test("#[cfg(all(test, feature = \"foo\"))] fn f() {}"), Some("fn f() {}"));
    }

    #[test]
    fn test_strip_cfg_test_no_bracket_returns_none() {
        assert_eq!(strip_cfg_test("#[cfg(test) fn f() {}"), None);
    }

    // --- parse_lt_assign dedup ---

    #[test]
    fn test_lt_assign_replaces_previous_track() {
        let src = "let x = lt!(vec![1], \"l\");\nlet p = lt!(x.as_ptr(), \"l\");\nx = lt!(vec![2], \"l\");\ndrop(x);\n";
        let config = Config::new().add_safe_fn("drop");
        let errors = check_source_with_config(src, &config);
        assert!(errors.is_empty(), "x reassignment should replace old owner, no borrow conflict");
    }

    // --- parse_destructure_assign ---

    #[test]
    fn test_parse_destructure_assign_tuple() {
        assert_eq!(parse_destructure_assign("(a, b) = expr"), Some(vec!["a".to_string(), "b".to_string()]));
    }

    #[test]
    fn test_parse_destructure_assign_array() {
        assert_eq!(parse_destructure_assign("[a, b] = expr"), Some(vec!["a".to_string(), "b".to_string()]));
    }

    #[test]
    fn test_parse_destructure_assign_struct() {
        let vars = parse_destructure_assign("Struct { x, y } = expr").unwrap();
        assert_eq!(vars, vec!["x".to_string(), "y".to_string()]);
    }

    #[test]
    fn test_parse_destructure_assign_skips_let() {
        assert_eq!(parse_destructure_assign("let (a, b) = expr"), None);
    }

    #[test]
    fn test_parse_destructure_assign_skips_for() {
        assert_eq!(parse_destructure_assign("for (a, b) in expr"), None);
    }

    // --- find_non_relational_eq ---

    #[test]
    fn test_find_non_relational_eq_simple() {
        assert_eq!(find_non_relational_eq("x = 1"), Some(2));
    }

    #[test]
    fn test_find_non_relational_eq_skips_compare() {
        assert_eq!(find_non_relational_eq("x == 1"), None);
        assert_eq!(find_non_relational_eq("x != 1"), None);
    }

    #[test]
    fn test_find_non_relational_eq_skips_string() {
        let s = r#"x "=" = 1"#;
        assert_eq!(find_non_relational_eq(s), Some(6));
    }

    // --- check_source with string in expression ---

    #[test]
    fn test_check_source_label_with_paren() {
        let src = r#"let val = lt!(vec![1], "foo)bar"); let p = lt!(val.as_ptr(), "foo)bar"); my_fn(val);"#;
        let errors = check_source_with_config(src, &Config::default());
        assert!(!errors.is_empty(), "label with ) inside should not break owner/borrow pairing");
    }

    #[test]
    fn test_check_source_comma_in_nested_vec() {
        let src = r#"let val = lt!(vec![1, 2], "l"); let p = lt!(val.as_ptr(), "l"); my_fn(val);"#;
        let errors = check_source_with_config(src, &Config::default());
        assert!(!errors.is_empty(), "comma in nested vec![1,2] should not break label parsing");
    }

    // --- is_ident ---

    #[test]
    fn test_is_ident_valid() {
        assert!(is_ident("foo"));
        assert!(is_ident("_bar"));
        assert!(is_ident("baz123"));
    }

    #[test]
    fn test_is_ident_invalid() {
        assert!(!is_ident(""));
        assert!(!is_ident("123abc"));
        assert!(!is_ident("foo bar"));
    }

    // --- strip_config ---

    #[test]
    fn test_strip_config_removes_config_flag() {
        let args = vec!["prog".to_string(), "--config".to_string(), "path.toml".to_string(), "check".to_string()];
        assert_eq!(strip_config(&args), vec!["prog", "check"]);
    }

    #[test]
    fn test_strip_config_no_config() {
        let args = vec!["prog".to_string(), "check".to_string(), "file.rs".to_string()];
        assert_eq!(strip_config(&args), vec!["prog", "check", "file.rs"]);
    }

    #[test]
    fn test_strip_config_multiple_config() {
        let args = vec!["prog".to_string(), "--config".to_string(), "a.toml".to_string(), "check".to_string(), "--config".to_string(), "b.toml".to_string()];
        assert_eq!(strip_config(&args), vec!["prog", "check"]);
    }

    // --- load_config ---

    #[test]
    fn test_load_config_no_args_returns_default() {
        let args: Vec<String> = vec!["prog".to_string()];
        let config = load_config(&args).unwrap();
        assert!(!config.is_safe_fn("some_fn"));
        assert!(!config.is_pointer_expr("some_ptr()"));
    }

    #[test]
    fn test_load_config_with_valid_config() {
        let dir = std::env::temp_dir();
        let config_path = dir.join("_test_load_config_args_valid.toml");
        let mut f = std::fs::File::create(&config_path).unwrap();
        write!(f, "safe_fn: custom_fn\nprefix: Ctx::new(\n").unwrap();
        let args = vec!["prog".to_string(), "--config".to_string(), config_path.to_str().unwrap().to_string(), "check".to_string()];
        let config = load_config(&args).unwrap();
        assert!(config.is_safe_fn("custom_fn"));
        assert!(config.is_pointer_expr("Ctx::new(x)"));
        std::fs::remove_file(&config_path).unwrap();
    }

    #[test]
    fn test_load_config_invalid_path_returns_err() {
        let args = vec!["prog".to_string(), "--config".to_string(), "/nonexistent/path.toml".to_string()];
        let result = load_config(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_config_missing_path_returns_err() {
        let args = vec!["prog".to_string(), "--config".to_string()];
        let result = load_config(&args);
        assert!(result.is_err());
    }
}
