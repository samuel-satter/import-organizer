use std::{collections::HashMap, env, fs, io, path::Path};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = get_file_path(&args)?;
    let content = fs::read_to_string(file_path)?;
    let organized = organize_rust_imports(&content);
    fs::write(file_path, organized)?;
    println!("Imports organized successfully.");
    Ok(())
}

fn get_file_path(args: &[String]) -> io::Result<&Path> {
    args.get(1)
        .map(Path::new)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Usage: imporg <file>"))
}

fn organize_rust_imports(code: &str) -> String {
    #[derive(Default)]
    struct ImportGroups<'a> {
        std_lib: Vec<&'a str>,
        external: Vec<&'a str>,
        internal: Vec<&'a str>,
    }

    let mut imports = ImportGroups::default();
    let mut non_import_lines = Vec::new();
    let mut in_import_section = false;

    for line in code.lines() {
        let trimmed = line.trim();
        match trimmed {
            s if s.starts_with("use ") => {
                in_import_section = true;
                let import = if s.starts_with("use std::") {
                    &mut imports.std_lib
                } else if s.starts_with("use crate::") {
                    &mut imports.internal
                } else {
                    &mut imports.external
                };
                import.push(s);
            }
            s if !s.is_empty() && in_import_section => {
                in_import_section = false;
                non_import_lines.push(line);
            }
            _ if !in_import_section => {
                non_import_lines.push(line);
            }
            _ => {}
        }
    }

    fn group_imports<'a>(imports: &[&'a str]) -> HashMap<&'a str, Vec<&'a str>> {
        imports.iter().fold(HashMap::new(), |mut acc, &import| {
            let (key, value) = import.split_once("::").unwrap_or((import, ""));
            acc.entry(key)
                .or_insert_with(Vec::new)
                .push(value.trim_start_matches("::"));
            acc
        })
    }

    fn format_group(key: &str, values: &[&str]) -> String {
        if values.len() == 1 && values[0].is_empty() {
            format!("use {};", key)
        } else if values.len() == 1 {
            format!("use {}::{};", key, values[0])
        } else {
            let mut result = String::with_capacity(
                key.len() + values.iter().map(|s| s.len()).sum::<usize>() + 20,
            );
            result.push_str("use ");
            result.push_str(key);
            result.push_str("::{");
            for (i, &value) in values.iter().enumerate() {
                if i > 0 {
                    result.push_str(",\n    ");
                }
                result.push_str(value);
            }
            result.push_str("};");
            result
        }
    }

    let mut organized = Vec::new();
    for (category, imports) in [
        ("Standard library", &imports.std_lib),
        ("External crate", &imports.external),
        ("Internal crate", &imports.internal),
    ] {
        if !imports.is_empty() {
            organized.push(format!("// {} imports", category));
            let grouped = group_imports(imports);
            for (key, values) in grouped.iter() {
                organized.push(format_group(key, values));
            }
            organized.push(String::new());
        }
    }

    organized.extend(non_import_lines.iter().map(|&s| s.to_string()));
    organized.join("\n")
}
