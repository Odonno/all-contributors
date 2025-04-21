use color_eyre::eyre::{OptionExt, Result, eyre};
use convert_case::{Case, Casing};
use regex::Regex;
use std::{collections::HashMap, fs};

const BUILD_CONTRIBUTION_TYPES: bool = false;

fn main() -> Result<()> {
    if BUILD_CONTRIBUTION_TYPES {
        generate_contribution_kinds_and_types()?;
    }

    Ok(())
}

fn generate_contribution_kinds_and_types() -> Result<()> {
    const SOURCE_FILENAME: &str = "src/models/contribution_table.html";

    let html_table = fs::read_to_string(SOURCE_FILENAME)?;

    let table =
        table_extract::Table::find_first(&html_table).ok_or_eyre("Cannot find HTML table..")?;

    let mut values = vec![];

    let code_regex = Regex::new(r"<code>(.+)</code>")?;

    for row in &table {
        let mut row_value = HashMap::new();

        let first_cell = row.get("Emoji/Type").ok_or_eyre("No emoji/type found..")?;

        let emoji = first_cell.chars().next().ok_or_eyre("No emoji found..")?;
        let emoji = String::from(emoji);

        let Some(captures) = code_regex.captures(first_cell) else {
            return Err(eyre!("No code found.."));
        };
        let code = captures.get(1).unwrap().as_str();

        let title = row.get("Represents").ok_or_eyre("No title found..")?;
        let description = row.get("Comments").unwrap_or("");

        row_value.insert("code", code.to_string());
        row_value.insert("emoji", emoji);
        row_value.insert("title", title.to_string());
        row_value.insert("description", description.to_string());

        values.push(row_value);
    }

    {
        const DEST_FILENAME: &str = "src/models/contribution_kind_gen.rs";

        let mut content = String::new();

        write_line(&mut content, "use color_eyre::eyre::{Error, Result, eyre};");
        write_line(&mut content, "use std::fmt;");
        write_line(&mut content, "");

        write_line(&mut content, "#[derive(Debug, Clone)]");
        write_line(&mut content, "pub enum ContributionKind {");

        for value in &values {
            let code = value.get("code").unwrap();
            let enum_value = code.to_case(Case::Pascal);

            content.push('\t');
            content.push_str(&enum_value);
            write_line(&mut content, ",");
        }

        write_line(&mut content, "}");

        write_line(&mut content, "");

        write_line(&mut content, "impl TryFrom<&str> for ContributionKind {");
        write_line(&mut content, "\ttype Error = Error;");
        write_line(&mut content, "");
        write_line(&mut content, "\tfn try_from(code: &str) -> Result<Self> {");
        write_line(&mut content, "\t\tmatch code {");

        for value in &values {
            let code = value.get("code").unwrap();
            let enum_value = code.to_case(Case::Pascal);
            let kind = format!("ContributionKind::{}", enum_value);

            content.push('\t');
            content.push('\t');
            content.push('\t');
            content.push('"');
            content.push_str(code);
            content.push('"');
            content.push_str(" => Ok(");
            content.push_str(&kind);
            write_line(&mut content, "),");
        }

        content.push('\t');
        content.push('\t');
        content.push('\t');
        write_line(
            &mut content,
            r#"_ => Err(eyre!("The contribution type '{}' does not exist.", code))"#,
        );

        write_line(&mut content, "\t\t}");
        write_line(&mut content, "\t}");
        write_line(&mut content, "}");

        write_line(&mut content, "");

        write_line(&mut content, "impl fmt::Display for ContributionKind {");
        write_line(
            &mut content,
            "\tfn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {",
        );
        write_line(&mut content, "\t\tmatch self {");

        for value in &values {
            let code = value.get("code").unwrap();
            let enum_value = code.to_case(Case::Pascal);
            let kind = format!("ContributionKind::{}", enum_value);

            content.push('\t');
            content.push('\t');
            content.push('\t');
            content.push_str(&kind);
            content.push_str(" => write!(f, ");
            content.push('"');
            content.push_str(code);
            content.push('"');
            write_line(&mut content, "),");
        }

        write_line(&mut content, "\t\t}");
        write_line(&mut content, "\t}");
        write_line(&mut content, "}");

        fs::write(DEST_FILENAME, content)?;
    }

    {
        const DEST_FILENAME: &str = "src/models/contribution_type_gen.rs";

        let mut content = String::new();

        write_line(
            &mut content,
            "use super::{ContributionKind, ContributionType};",
        );
        write_line(&mut content, "");
        write_line(
            &mut content,
            "pub fn get_contribution_type_from_kind(kind: ContributionKind) -> Option<ContributionType> {",
        );

        write_line(&mut content, "\tmatch kind {");

        for value in &values {
            content.push('\t');
            content.push('\t');

            let code = value.get("code").unwrap();
            let enum_value = code.to_case(Case::Pascal);
            let kind = format!("ContributionKind::{}", enum_value);

            content.push_str(&kind);
            content.push_str(" => Some(ContributionType { ");

            content.push_str("kind: ");
            content.push_str(&kind);
            content.push_str(", ");

            content.push_str("code: String::from(");
            content.push('"');
            content.push_str(value.get("code").unwrap());
            content.push('"');
            content.push_str("), ");

            content.push_str("emoji: String::from(");
            content.push('"');
            content.push_str(value.get("emoji").unwrap());
            content.push('"');
            content.push_str("), ");

            content.push_str("title: String::from(");
            content.push('"');
            content.push_str(value.get("title").unwrap());
            content.push('"');
            content.push_str("), ");

            content.push_str("description: String::from(");
            content.push('"');
            content.push_str(value.get("description").unwrap());
            content.push('"');
            content.push_str("), ");

            content.push_str(" }),");

            write_line(&mut content, "");
        }

        write_line(&mut content, "\t\t_ => None,");
        write_line(&mut content, "\t}");
        write_line(&mut content, "}");

        fs::write(DEST_FILENAME, content)?;
    }

    Ok(())
}

fn write_line(new_content: &mut String, line: &str) {
    new_content.push_str(line);
    new_content.push('\n');
}
