use std::io::Write;
use std::{
    error::Error,
    fs::{self, OpenOptions},
    path::PathBuf,
};

use chrono::Utc;

use crate::models::data::Data;
use crate::{handlers::handlers::FormattedJob, models::scraper::Job};

pub enum ReportMode {
    HTML,
    CSV,
}

fn append_jobs_to_html(jobs: &Vec<FormattedJob>, html: String) -> String {
    // let document = scraper::Html::parse_document(html);
    // let tbody_selector = scraper::Selector::parse("tbody").unwrap();

    let mut html = html.to_string();

    // Find the position right after <tbody>
    if let Some(tbody_pos) = html.find("<tbody>") {
        let insert_pos = tbody_pos + "<tbody>".len();

        // Create the new rows HTML
        let mut new_rows = String::new();
        for fj in jobs {
            new_rows.push_str(&format!(
                r#"<tr><td>{}</td> <td>{}</td> <td>{}</td> <td><a href="{}">{}</a></td></tr>"#,
                fj.company, fj.job.title, fj.job.location, fj.job.link, "Apply"
            ));
        }

        // Insert the new rows after <tbody>
        html.insert_str(insert_pos, &new_rows);
    }

    html
}

pub fn create_report(new_jobs: &Vec<FormattedJob>, mode: ReportMode) -> Result<(), Box<dyn Error>> {
    let today = Utc::now().naive_utc().date().to_string();

    let mut path = Data::get_data_dir();

    dbg!(&path);

    if cfg!(test) {
        path.push("tests");
        if !fs::exists(&path)? {
            fs::create_dir(&path)?;
        }
    }
    path.push("reports");

    if !fs::exists(&path)? {
        fs::create_dir(&path)?;
    }
    match mode {
        ReportMode::CSV => {
            let names_row = "Company,Title,Location,Link\n";
            let entries = new_jobs
                .iter()
                .map(|j| {
                    format!(
                        "{},{},{},{}\n",
                        j.company,
                        j.job.title,
                        j.job.location.replace(",", ""),
                        j.job.link
                    )
                })
                .collect::<String>();
            let csv = format!("{}{}", names_row, entries);
            // check if the root path exists
            path.push(today + ".csv");

            if fs::exists(&path)? {
                let mut file = OpenOptions::new().append(true).open(&path)?;

                write!(file, "{}", entries)?;
            } else {
                fs::write(&path, format!("{}", csv))?;
            }
        }
        ReportMode::HTML => {
            path.push(today.clone() + ".html");

            if fs::exists(&path)? {
                let prev_html = fs::read_to_string(&path).unwrap();
                let modified_html = append_jobs_to_html(new_jobs, prev_html);
                fs::write(&path, modified_html)?;
            } else {
                let html = format!(
                r#"
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title></title>
  </head>

  <body>
    <h1>{}</h1>
    <table>
      <thead>
        <tr>
          <th>Company</th>
          <th>Title</th>
          <th>Location</th>
          <th>Link</th>
        </tr>
      </thead>

      <tbody>
        {}
      </tbody>
    </table>
  </body>
</html>
"#,
                format!("New Jobs: {}", today),
                new_jobs
                    .iter()
                    .map(|fj| {
                        format!(
                            r#"<tr><td>{}</td> <td>{}</td> <td>{}</td> <td><a href="{}">Apply</a></td></tr>"#,
                            fj.company, fj.job.title, fj.job.location, fj.job.link
                        )
                    })
                    .collect::<String>()
            );

                fs::write(&path, html)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::models::scraper::Job;

    use super::*;

    #[test]
    fn test_create_report() {
        let v = create_report(
            &vec![FormattedJob {
                display_name: "SWE".to_string(),
                company: "Disney".to_string(),
                job: Job {
                    title: "Software Engineer".to_string(),
                    link: "https://somelinktoapply.com".to_string(),
                    location: "Anaheim, CA".to_string(),
                    id: Uuid::new_v4(),
                    is_seen: false,
                    applied: false,
                    is_bookmarked: false,
                },
            }],
            ReportMode::HTML,
        );

        if let Err(e) = &v {
            println!("Error: {}", e);
        }
        assert_eq!(v.is_ok(), true);
    }
}
