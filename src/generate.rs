use color_eyre::eyre::{Result, eyre};
use std::{fs, path::Path};

use crate::models::{ContributionKind, ContributionType, Contributor, ContributorsConfig};

pub fn main() -> Result<()> {
    const CONTRIBUTORS_CONFIG_FILENAME: &str = ".all-contributorsrc";

    let exists = fs::exists(CONTRIBUTORS_CONFIG_FILENAME)?;

    if !exists {
        return Err(eyre!(
            "The configuration file '{}' does not exist",
            CONTRIBUTORS_CONFIG_FILENAME
        ));
    }

    let config_str = fs::read_to_string(CONTRIBUTORS_CONFIG_FILENAME)?;
    let config: ContributorsConfig = serde_json::from_str(&config_str)?;

    for filename in &config.files {
        let exists = fs::exists(filename)?;

        if !exists {
            return Err(eyre!("The file '{}' does not exist", filename));
        }

        let path = Path::new(filename);

        update_contributors_list(&path, &config)?;
    }

    Ok(())
}

fn update_contributors_list(path: &Path, config: &ContributorsConfig) -> Result<()> {
    const START_LINE_STR: &str =
        "<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->";
    const END_LINE_STR: &str = "<!-- ALL-CONTRIBUTORS-LIST:END -->";

    let content = fs::read_to_string(path)?;

    let mut new_content = String::with_capacity(content.len());

    let mut line_start: Option<_> = None;
    let mut line_end: Option<_> = None;

    let lines = content.lines();

    for (index, line) in lines.enumerate() {
        if line_start.is_none() && line == START_LINE_STR {
            line_start = Some(index);
            continue;
        }

        if line_start.is_some() && line_end.is_none() {
            if line == END_LINE_STR {
                line_end = Some(index);

                write_line(&mut new_content, START_LINE_STR);
                generate_contributors_table(&mut new_content, &config)?;
                write_line(&mut new_content, END_LINE_STR);
            }
            continue;
        }

        write_line(&mut new_content, line);
    }

    fs::write(path, new_content)?;

    Ok(())
}

fn generate_contributors_table(
    new_content: &mut String,
    config: &ContributorsConfig,
) -> Result<()> {
    write_line(new_content, "<!-- prettier-ignore-start -->");
    write_line(new_content, "<!-- markdownlint-disable -->");

    write_line(new_content, "<table>");
    write_line(new_content, "\t<tbody>");

    let width = 100.0 / config.contributors_per_line as f32;
    let width = format!("{:.2}", width);

    for row in config
        .contributors
        .chunks(config.contributors_per_line as usize)
    {
        write_line(new_content, "\t\t<tr>");

        for contributor in row {
            new_content.push_str("\t\t\t");
            new_content.push_str(r#"<td align="center" valign="top" width="#);
            new_content.push('"');
            new_content.push_str(&width);
            new_content.push('%');
            new_content.push('"');
            new_content.push_str(">");

            // generate link with image & all
            new_content.push_str("<a href=");
            new_content.push('"');
            new_content.push_str(&contributor.profile);
            new_content.push('"');
            new_content.push_str(">");

            // generate img
            new_content.push_str("<img src=");
            new_content.push('"');
            new_content.push_str(&contributor.avatar_url);
            new_content.push_str("?s=");
            new_content.push_str(&format!("{}", config.image_size));
            new_content.push('"');
            new_content.push_str(" width=");
            new_content.push('"');
            new_content.push_str(&format!("{}", config.image_size));
            new_content.push_str("px;");
            new_content.push('"');
            new_content.push_str(" alt=");
            new_content.push('"');
            new_content.push_str(&contributor.name);
            new_content.push('"');
            new_content.push_str(">");

            new_content.push_str("<br />");

            new_content.push_str("<sub>");
            new_content.push_str("<b>");
            new_content.push_str(&contributor.name);
            new_content.push_str("</b>");
            new_content.push_str("</sub>");

            new_content.push_str("</a>");

            new_content.push_str("<br />");

            // generate contributions list per contributor
            for (index, contribution_code) in contributor.contributions.iter().enumerate() {
                let kind = ContributionKind::try_from(contribution_code.to_string())?;
                let contribution_type = ContributionType::try_from(kind.clone())?;

                let link = generate_link(kind, config, contributor);

                new_content.push_str("<a href=");
                new_content.push('"');
                new_content.push_str(&link);
                new_content.push('"');
                new_content.push_str(" title=");
                new_content.push('"');
                new_content.push_str(&contribution_type.title);
                new_content.push('"');
                new_content.push_str(">");
                new_content.push_str(&contribution_type.emoji);
                new_content.push_str("</a>");

                if index < contributor.contributions.len() - 1 {
                    new_content.push(' ');
                }
            }

            new_content.push_str("</td>");
            new_content.push('\n');
        }

        write_line(new_content, "\t\t</tr>");
    }

    write_line(new_content, "\t</tbody>");
    write_line(new_content, "</table>");

    write_line(new_content, "");
    write_line(new_content, "<!-- markdownlint-restore -->");
    write_line(new_content, "<!-- prettier-ignore-end -->");
    write_line(new_content, "");

    Ok(())
}

fn generate_link(
    kind: ContributionKind,
    config: &ContributorsConfig,
    contributor: &Contributor,
) -> String {
    match kind {
        ContributionKind::Bug | ContributionKind::Doc => {
            generate_link_to_issues(config, contributor)
        }
        ContributionKind::Code | ContributionKind::Test => {
            generate_link_to_commits(config, contributor)
        }
        ContributionKind::Review => generate_link_to_pr_reviews(config, contributor),
        _ => format!("#{}-{}", kind, contributor.login),
    }
}

fn generate_link_to_issues(config: &ContributorsConfig, contributor: &Contributor) -> String {
    format!(
        "{}/{}/{}/issues?q=author%3A{}",
        config.repo_host, config.project_owner, config.project_name, contributor.login
    )
}

fn generate_link_to_commits(config: &ContributorsConfig, contributor: &Contributor) -> String {
    format!(
        "{}/{}/{}/commits?author={}",
        config.repo_host, config.project_owner, config.project_name, contributor.login
    )
}

fn generate_link_to_pr_reviews(config: &ContributorsConfig, contributor: &Contributor) -> String {
    format!(
        "{}/{}/{}/pulls?q=is%3Apr+reviewed-by%3A{}",
        config.repo_host, config.project_owner, config.project_name, contributor.login
    )
}

fn write_line(new_content: &mut String, line: &str) {
    new_content.push_str(line);
    new_content.push('\n');
}

fn update_contributors_badge(path: &Path) -> Result<()> {
    todo!();
}
