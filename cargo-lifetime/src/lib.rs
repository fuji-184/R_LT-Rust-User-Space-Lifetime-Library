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
    "move", "ref", "mut",
];

fn has_unclosed_lt(s: &str) -> bool {
    let mut in_str = false;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < s.len() {
        if bytes[i] == b'"' {
            in_str = !in_str;
            i += 1;
            continue;
        }
        if in_str {
            i += 1;
            continue;
        }
        if i + 4 <= s.len() && &s[i..i + 4] == "lt!(" {
            let mut depth: isize = 1;
            let mut j = i + 4;
            let mut found_close = false;
            while j < s.len() {
                match bytes[j] {
                    b'(' => depth += 1,
                    b')' => {
                        depth -= 1;
                        if depth == 0 {
                            found_close = true;
                            j += 1;
                            break;
                        }
                    }
                    b'"' => {
                        j += 1;
                        while j < s.len() && bytes[j] != b'"' {
                            if bytes[j] == b'\\' { j += 1; }
                            j += 1;
                        }
                        if j < s.len() { j += 1; }
                        continue;
                    }
                    _ => {}
                }
                j += 1;
            }
            if !found_close {
                return true;
            }
            i = j;
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

    if let Some(p) = pending {
        result.push(p);
    }

    result
}

fn strip_comment(s: &str) -> &str {
    if let Some(pos) = s.find("//") {
        let before = &s[..pos].trim_end();
        if before.is_empty() { "" } else { before }
    } else {
        s
    }
}

pub fn check_source(src: &str) -> Vec<(usize, String)> {
    let mut sc = Scanner::new();
    let lines = join_lt_lines(src);

    for &(line_num, ref raw) in &lines {
        let s = raw.trim();
        if s.is_empty() {
            continue;
        }

        if s.starts_with("#[cfg(test)]") {
            let rest = s["#[cfg(test)]".len()..].trim();
            if rest.is_empty() {
                continue;
            }
        }

        let mut seg_start = 0;
        let mut in_str = false;
        for (j, ch) in s.char_indices() {
            if ch == '"' {
                in_str = !in_str;
                continue;
            }
            if in_str {
                continue;
            }
            if ch == '}' || ch == '{' {
                let seg = s[seg_start..j].trim();
                let seg = strip_comment(seg);
                if !seg.is_empty() {
                    for sub in seg.split(';') {
                        let sub = sub.trim();
                        if !sub.is_empty() {
                            sc.analyze_segment(sub, line_num);
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
        let seg = strip_comment(seg);
        if !seg.is_empty() {
            for sub in seg.split(';') {
                let sub = sub.trim();
                if !sub.is_empty() {
                    sc.analyze_segment(sub, line_num);
                }
            }
        }
    }

    std::mem::take(&mut sc.errors)
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
        if b[i] == b'[' {
            if let Some(end) = msg[i..].find(']') {
                r.push_str(YELLOW);
                r.push_str(&msg[i..=i + end]);
                r.push_str(RESET);
                i += end + 1;
                continue;
            }
        }
        if b[i] == b'`' {
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
        r.push(msg[i..].chars().next().unwrap());
        i += msg[i..].chars().next().unwrap().len_utf8();
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
    depth: usize,
    var_depth: HashMap<String, usize>,
    owners: Vec<Owner>,
    borrows: Vec<Borrow>,
    errors: Vec<(usize, String)>,
}

impl Scanner {
    fn new() -> Self {
        Scanner {
            depth: 0,
            var_depth: HashMap::new(),
            owners: Vec::new(),
            borrows: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn analyze_segment(&mut self, code: &str, line: usize) {
        if let Some((var, expr, label)) = parse_lt_let(code) {
            self.owners.retain(|o| !(o.var == var && o.depth == self.depth));
            self.borrows.retain(|b| !(b.var == var && b.depth == self.depth));
            self.var_depth.insert(var.clone(), self.depth);
            if is_pointer_expr(&expr) {
                self.borrows.push(Borrow { var, label, line, depth: self.depth });
            } else {
                self.owners.push(Owner { var, label, line, depth: self.depth });
            }
            return;
        }

        if let Some(vars) = parse_destructure(code) {
            for var in vars {
                self.owners.retain(|o| !(o.var == var && o.depth == self.depth));
                self.borrows.retain(|b| !(b.var == var && b.depth == self.depth));
                self.var_depth.insert(var, self.depth);
            }
            return;
        }

        if let Some(var) = parse_let(code) {
            self.owners.retain(|o| !(o.var == var && o.depth == self.depth));
            self.borrows.retain(|b| !(b.var == var && b.depth == self.depth));
            self.var_depth.insert(var, self.depth);
            return;
        }

        if let Some(var) = parse_for(code) {
            self.owners.retain(|o| !(o.var == var && o.depth == self.depth));
            self.borrows.retain(|b| !(b.var == var && b.depth == self.depth));
            self.var_depth.insert(var, self.depth);
            return;
        }

        if let Some((var, _expr, label)) = parse_lt_assign(code) {
            let decl_d = self.var_depth.get(&var).copied().unwrap_or(self.depth);
            let eff_d = decl_d.min(self.depth);
            self.borrows.push(Borrow { var, label, line, depth: eff_d });
            return;
        }

        if let Some(var) = parse_drop(code) {
            self.check_drop(&var, line);
            return;
        }

        self.check_owner_used(code, line);
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
                if b.label != o.label {
                    continue;
                }
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
        if fn_name.contains(' ') || fn_name.contains('=') || fn_name.is_empty() {
            return;
        }
        if is_safe_fn(fn_name) {
            return;
        }
        if KEYWORDS.contains(&fn_name) {
            return;
        }
        let args = match extract_paren_body(&c[lparen..]) {
            Some(v) => v,
            None => return,
        };
        for arg in split_top_level_commas(args) {
            let arg = arg.trim();
            if !is_ident(arg) {
                continue;
            }
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
            if o.var != var {
                continue;
            }
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
    if !s.starts_with('(') {
        return None;
    }
    let mut depth: isize = 0;
    for (i, ch) in s.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&s[1..i]);
                }
            }
            _ => {}
        }
    }
    None
}

fn split_top_level_commas(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut depth: isize = 0;
    let mut in_str = false;
    for (i, ch) in s.char_indices() {
        if ch == '"' {
            in_str = !in_str;
        } else if !in_str {
            match ch {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => depth -= 1,
                ',' if depth == 0 => {
                    parts.push(&s[start..i]);
                    start = i + 1;
                }
                _ => {}
            }
        }
    }
    parts.push(&s[start..]);
    parts
}

fn parse_lt_let(code: &str) -> Option<(String, String, String)> {
    let c = code.trim();
    if !c.starts_with("let ") {
        return None;
    }
    let rest = &c[4..];
    let mut word = rest.trim_start();
    loop {
        let end = word
            .find(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
            .unwrap_or(word.len());
        let token = &word[..end];
        if token == "mut" || token == "ref" {
            word = word[end..].trim_start();
            continue;
        }
        if token.is_empty() || !is_ident(token) {
            return None;
        }
        let eq = word[end..].find('=')?;
        let var = token.to_string();
        let after = word[end + eq + 1..].trim();
        return extract_lt_expr_label(after)
            .map(|(expr, label)| (var, expr, label));
    }
}

fn parse_let(code: &str) -> Option<String> {
    let c = code.trim();
    if !c.starts_with("let ") {
        return None;
    }
    let rest = &c[4..];
    let mut word = rest.trim_start();
    loop {
        let end = word
            .find(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
            .unwrap_or(word.len());
        let token = &word[..end];
        if token == "mut" || token == "ref" {
            word = word[end..].trim_start();
            continue;
        }
        if token.is_empty() || !is_ident(token) {
            return None;
        }
        return Some(token.to_string());
    }
}

fn parse_for(code: &str) -> Option<String> {
    let c = code.trim();
    if !c.starts_with("for ") {
        return None;
    }
    let rest = &c[4..].trim_start();
    if rest.is_empty() {
        return None;
    }
    let next = rest.split(|ch: char| ch.is_whitespace() || ch == '(').next()?;
    if is_ident(next) {
        Some(next.to_string())
    } else {
        None
    }
}

fn parse_destructure(code: &str) -> Option<Vec<String>> {
    let c = code.trim();
    if !c.starts_with("let ") {
        return None;
    }
    let mut rest = &c[4..];
    loop {
        let trimmed = rest.trim_start();
        if trimmed.starts_with("mut ") {
            rest = &trimmed[4..];
        } else if trimmed.starts_with("ref ") {
            rest = &trimmed[4..];
        } else {
            rest = trimmed;
            break;
        }
    }
    if rest.is_empty() {
        return None;
    }

    let mut vars = Vec::new();

    if rest.starts_with('(') {
        if let Some(body) = extract_paren_body(rest) {
            for part in split_top_level_commas(body) {
                let part = part.trim();
                if is_ident(part) {
                    vars.push(part.to_string());
                }
            }
        }
    } else if rest.starts_with("Some(") || rest.starts_with("Ok(") || rest.starts_with("Err(") {
        let paren_start = rest.find('(')?;
        if let Some(body) = extract_paren_body(&rest[paren_start..]) {
            let body = body.trim();
            if is_ident(body) {
                vars.push(body.to_string());
            }
        }
    } else if rest.starts_with('&') {
        let after_ref = rest[1..].trim_start();
        if after_ref.starts_with("mut ") {
            let after_mut = &after_ref[4..].trim_start();
            if after_mut.starts_with('(') {
                return parse_destructure(&format!("let {}", after_mut));
            }
            if is_ident(after_mut) {
                vars.push(after_mut.to_string());
            }
        } else if after_ref.starts_with('(') {
            return parse_destructure(&format!("let {}", after_ref));
        } else if is_ident(after_ref) {
            vars.push(after_ref.to_string());
        }
    }

    if vars.is_empty() { None } else { Some(vars) }
}

fn parse_lt_assign(code: &str) -> Option<(String, String, String)> {
    let c = code.trim();
    if c.starts_with("let ") {
        return None;
    }
    let eq = find_non_relational_eq(c)?;
    let var = c[..eq].trim();
    if !is_ident(var) {
        return None;
    }
    let after = c[eq + 1..].trim();
    extract_lt_expr_label(after).map(|(expr, label)| (var.to_string(), expr, label))
}

fn find_non_relational_eq(s: &str) -> Option<usize> {
    let mut in_str = false;
    for (i, ch) in s.char_indices() {
        if ch == '"' {
            in_str = !in_str;
        } else if !in_str && ch == '=' {
            if i > 0 {
                let prev = s.as_bytes()[i - 1];
                if prev == b'!' || prev == b'<' || prev == b'>' || prev == b'=' {
                    continue;
                }
            }
            if s.as_bytes().get(i + 1) == Some(&b'=') {
                continue;
            }
            return Some(i);
        }
    }
    None
}

fn extract_lt_expr_label(s: &str) -> Option<(String, String)> {
    if !s.starts_with("lt!(") {
        return None;
    }
    let body_start = &s[4..];
    let mut depth: isize = 0;
    let mut end = None;
    for (i, ch) in body_start.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' if depth == 0 => {
                end = Some(i);
                break;
            }
            ')' => depth -= 1,
            _ => {}
        }
    }
    let end = end?;
    let body = body_start[..end].trim();

    let last_comma = find_last_comma(body)?;
    let expr = body[..last_comma].trim().to_string();
    let label_raw = body[last_comma + 1..].trim();

    if label_raw.len() < 2 || !label_raw.starts_with('"') || !label_raw.ends_with('"') {
        return None;
    }
    let label = label_raw[1..label_raw.len() - 1].to_string();
    Some((expr, label))
}

fn find_last_comma(s: &str) -> Option<usize> {
    let mut in_str = false;
    let mut nest: isize = 0;
    let mut last = None;
    for (i, ch) in s.char_indices() {
        if ch == '"' {
            in_str = !in_str;
        } else if !in_str {
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
    if !c.starts_with("drop(") {
        return None;
    }
    let rest = &c[5..];
    let paren = rest.find(')')?;
    let var = rest[..paren].trim();
    if var.is_empty() {
        return None;
    }
    Some(var.to_string())
}

fn is_safe_fn(name: &str) -> bool {
    matches!(
        name,
        "println" | "print" | "eprintln" | "eprint" | "write" | "writeln"
            | "format" | "format_args"
            | "assert" | "assert_eq" | "assert_ne"
            | "debug_assert" | "debug_assert_eq" | "debug_assert_ne"
            | "panic" | "unreachable" | "unimplemented" | "todo"
            | "dbg" | "stringify" | "concat"
            | "Vec::new" | "vec" | "Box::new" | "Rc::new" | "Arc::new"
            | "String::new" | "String::from" | "format!"
            | "Some" | "Ok" | "Err" | "None"
            | "std::mem::drop" | "mem::drop" | "drop"
    )
}

fn is_pointer_expr(e: &str) -> bool {
    let e = e.trim();
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
        || e.starts_with("std::ptr::NonNull::new_unchecked(");
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
        || e.ends_with(".borrow_mut()");
    starts || ends
}

pub fn is_ident(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
