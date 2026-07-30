use lifetime_cli::*;

const LINE_OFFSET: usize = 1;

fn run_tests() {
    let test_dir = format!("{}/../tests", env!("CARGO_MANIFEST_DIR"));
    let mut cases: Vec<_> = std::fs::read_dir(&test_dir)
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
                let errors = check_source(&src);
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
            println!("  {GREEN}PASS{RESET}  tests/{name}");
            pass += 1;
        } else {
            println!("  {RED}FAIL{RESET}  tests/{name}");
            fail += 1;
        }
        for (line, msg) in errors {
            print!("        ");
            pline(name, *line, msg);
        }
    }

    let total = pass + fail;
    let n_cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    if fail > 0 {
        println!("---\n{GREEN}{pass}{RESET}/{total} passed, {RED}{fail}{RESET} failed  [{n_cores} cores]");
        std::process::exit(1);
    } else {
        println!("---\n{GREEN}{pass}{RESET}/{total} passed, {RED}{fail}{RESET} failed  [{n_cores} cores]");
    }
}

fn main() {
    run_tests();
}
