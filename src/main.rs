const LINE_OFFSET: usize = 1;

fn adjust_line_in_msg(msg: &str, n: usize) -> String {
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

fn run_tests() {
    let mut cases: Vec<_> = std::fs::read_dir("tests")
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|s| s == "rs").unwrap_or(false))
        .collect();
    cases.sort_by_key(|e| e.file_name());

    let results: Vec<(String, Vec<(usize, String)>, bool)> = std::thread::scope(|s| {
        let mut handles = Vec::with_capacity(cases.len());

        for entry in &cases {
            let name = entry.file_name().to_string_lossy().to_string();
            let body = std::fs::read_to_string(entry.path()).unwrap();
            let body_line_count = body.lines().count();
            let expect_err = name.starts_with("invalid_");

            handles.push(s.spawn(move || {
                let src = format!(
                    r#"macro_rules! lt {{ ($e:expr, $l:expr) => {{ $e }}; }}
fn main() {{ {} }}"#,
                    body
                );
                let errors = lifetime::check_source(&src);
                let errors: Vec<(usize, String)> = errors
                    .into_iter()
                    .map(|(line, msg)| {
                        let orig = if line > LINE_OFFSET {
                            (line - LINE_OFFSET).min(body_line_count)
                        } else {
                            1
                        };
                        let msg = adjust_line_in_msg(&msg, LINE_OFFSET);
                        (orig, msg)
                    })
                    .collect();
                let ok = if expect_err { !errors.is_empty() } else { errors.is_empty() };
                (name, errors, ok)
            }));
        }

        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });

    let mut pass = 0u32;
    let mut fail = 0u32;

    for (name, errors, ok) in &results {
        if *ok {
            println!("  PASS  tests/{name}");
            pass += 1;
        } else {
            println!("  FAIL  tests/{name}");
            fail += 1;
        }
        for (line, msg) in errors {
            println!("        tests/{name}:{line}: {msg}");
        }
    }

    let total = pass + fail;
    let n_cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    println!("---\n{pass}/{total} passed, {fail} failed  [{n_cores} cores]");
    if fail > 0 {
        std::process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("check") => {
            let path = args.get(2).map(|s| s.as_str()).unwrap_or("src/main.rs");
            let errors = lifetime::check_file(path);
            for (line, msg) in &errors {
                println!("{}:{}: {}", path, line, msg);
            }
            if !errors.is_empty() {
                std::process::exit(1);
            }
        }
        _ => run_tests(),
    }
}
