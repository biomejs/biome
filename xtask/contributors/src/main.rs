use pico_args::Arguments;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::path::PathBuf;
use xtask::glue::fs2;
use xtask::*;

/// A token is needed to run this script. To create a token, go to <https://github.com/settings/tokens>
/// and give it read access to the repository.
///
/// Only users that have read rights can run this script
fn main() -> Result<()> {
    let root = project_root().join("website/src/components");
    let mut args = Arguments::from_env();
    let token: String = args.value_from_str("--token").unwrap();
    let contributors = get_contributors(token.as_str());
    write_contributors_in_credits(root.clone(), contributors.as_slice())?;
    write_contributors_in_community(root.clone(), contributors.as_slice())?;

    Ok(())
}
const IMPORT_IMAGE: &str = "import { Image } from \"astro:assets\"";

fn write_contributors_in_community(root: PathBuf, contributors: &[Contributor]) -> Result<()> {
    let mut content = String::new();

    content.push_str(format!("---\n // {} \n {}  \n---\n", PREAMBLE, IMPORT_IMAGE).as_str());

    let contributors_per_row = [5, 4, 6, 5, 3, 5, 4];

    let mut current_index = 0;
    for items_per_row in contributors_per_row {
        let current_contributors = &contributors[current_index..current_index + items_per_row];
        writeln!(content, "<div class=\"contributor-row\">")?;
        for contributor in current_contributors {
            let person = format!("User {}", contributor.login);

            writeln!(
                content,
                "<Image
              class=\"contributor-avatar\"
              src={}
              alt=\"{}\"
              width=\"84\"
              height=\"84\"
        />",
                contributor.avatar_url, person
            )?;
        }
        current_index += items_per_row;

        writeln!(content, "</div>")?;
    }

    fs2::write(root.join("Community.astro"), content)?;
    Ok(())
}

fn write_contributors_in_credits(root: PathBuf, contributors: &[Contributor]) -> Result<()> {
    let mut content = String::new();

    content.push_str(format!("---\n // {} \n {}  \n---\n", PREAMBLE, IMPORT_IMAGE).as_str());

    content.push('\n');
    content.push_str("<h2>Code contributors</h2>");
    content.push('\n');
    content.push_str("<ul class=\"credits-people-list contributors\">");
    content.push('\n');
    for contributor in contributors {
        let mut contributor_html = String::new();
        let escaped_login = html_escape::encode_text(&contributor.login);
        let escaped_avatar = html_escape::encode_text(&contributor.avatar_url);
        contributor_html.push_str("<li><a href=\"https://github.com/biomejs/biome/commits?author=");

        html_escape::encode_double_quoted_attribute_to_string(
            format!("{}", escaped_login),
            &mut contributor_html,
        );
        contributor_html.push_str("\">");
        contributor_html.push_str("<Image width=\"150\" height=\"150\" src=\"");
        html_escape::encode_double_quoted_attribute_to_string(
            format!("{}", escaped_avatar),
            &mut contributor_html,
        );
        content.push_str(&contributor_html);
        write!(content, "\" alt=\"{}\" />", contributor.login)?;
        write!(content, "<span>{}</span>", escaped_login)?;
        content.push_str("</a></li>");
        content.push('\n');
    }

    content.push_str("</ul>");
    fs2::write(root.join("Contributors.astro"), content)?;
    Ok(())
}

#[derive(Debug, Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq)]
struct Contributor {
    avatar_url: String,
    login: String,
    id: u64,
    contributions: u64,
}

fn get_contributors(token: &str) -> Vec<Contributor> {
    let mut contributors = Vec::new();
    contributors_request(
        "https://api.github.com/repos/biomejs/biome/contributors",
        token,
        &mut contributors,
    );
    contributors.sort_by(|a, b| b.contributions.cmp(&a.contributions));
    contributors
}

fn contributors_request(url: &str, token: &str, contributors: &mut Vec<Contributor>) {
    let request = ureq::get(url)
        .set("User-Agent", "@biomejs")
        .set("Authorization", &format!("token {token}"));

    match request.call() {
        Ok(response) => {
            if let Some(link) = response.header("link") {
                if link.contains("rel=\"next\"") {
                    let start_index = link
                        .find("rel=\"prev\", ")
                        .map(|index| index + "rel=\"prev\", ".len())
                        .unwrap_or(0);
                    // SAFETY: checked before
                    let end_index = link.find("; rel=\"next\"").unwrap();
                    let url = &link[start_index..end_index];
                    let url = url.replace(['<', '>'], "");

                    contributors_request(&url, token, contributors);
                }
            }
            let result: Result<Vec<Contributor>, std::io::Error> = response.into_json();
            if let Ok(new_contributors) = result {
                contributors.extend(new_contributors);
            }
        }
        Err(err) => {
            eprintln!("{:?}", err);
        }
    }
}
