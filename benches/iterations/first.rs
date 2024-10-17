use std::collections::HashMap;

pub fn organize_rust_imports(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut imports = Vec::new();
    let mut organized = Vec::new();

    for line in &lines {
        if line.trim().starts_with("use ") {
            imports.push(line.trim());
        } else if !imports.is_empty() {
            break;
        }
    }

    let mut std_lib = Vec::new();
    let mut external = Vec::new();
    let mut internal = Vec::new();

    for import in imports {
        if import.starts_with("use std::") {
            std_lib.push(import);
        } else if import.starts_with("use crate::") {
            internal.push(import);
        } else {
            external.push(import);
        }
    }

    fn group_imports(imports: &[&str]) -> HashMap<String, Vec<String>> {
        let mut grouped = HashMap::new();
        for &import in imports {
            let parts: Vec<&str> = import.split("::").collect();
            let key = parts[..2].join("::");
            let value = parts[2..].join("::");
            grouped.entry(key).or_insert_with(Vec::new).push(value);
        }
        grouped
    }

    fn format_group((key, values): (&String, &Vec<String>)) -> String {
        if values.len() == 1 {
            format!("use {}{};", key, values[0])
        } else {
            let items = values.join(",\n    ");
            format!("use {} {{\n    {}\n}}", key, items)
        }
    }

    if !std_lib.is_empty() {
        organized.push("// Standard library imports".to_string());
        let grouped = group_imports(&std_lib);
        organized.extend(grouped.iter().map(format_group));
        organized.push(String::new());
    }

    if !external.is_empty() {
        organized.push("// External crate imports".to_string());
        let grouped = group_imports(&external);
        organized.extend(grouped.iter().map(format_group));
        organized.push(String::new());
    }

    if !internal.is_empty() {
        organized.push("// Internal crate imports".to_string());
        let grouped = group_imports(&internal);
        organized.extend(grouped.iter().map(format_group));
    }

    let mut result = Vec::new();
    let mut import_section_passed = false;

    for line in lines {
        if !import_section_passed {
            if line.trim().starts_with("use ") {
                if result.is_empty() {
                    result.append(&mut organized);
                }
                import_section_passed = true;
            } else {
                result.push(line.to_string());
            }
        } else {
            result.push(line.to_string());
        }
    }
    result.join("\n")
}