pub struct DefaultJobScraperOptions {
    pub content_selector: &'static str,
    pub headless: bool,
    pub company_key: &'static str,
    pub url: &'static str,
    pub get_jobs_js: &'static str,
}

pub const ANDURIL_SCRAPE_OPTIONS: DefaultJobScraperOptions = DefaultJobScraperOptions {
    headless: true,
    company_key: "Anduril",
    content_selector: "body",
    url: "https://www.anduril.com/open-roles?location=&department=Software&search=&gh_src=",
    get_jobs_js: r#"const jobs = [...document.querySelectorAll(".JobListing_jobListItems__lXfbo")].map(item => {
        const link = item.querySelector("a").href;
        const pTags = [...item.querySelectorAll("p")];
        return {
            title: pTags[0].innerText,
            location: pTags[1].innerText,
            link
        }
    })

    JSON.stringify(jobs);"#,
};

pub const ONEPASSWORD_SCRAPE_OPTIONS: DefaultJobScraperOptions = DefaultJobScraperOptions {
    headless: true,
    company_key: "1Password",
    content_selector: ".content",
    url: "https://jobs.lever.co/1password",
    get_jobs_js: r##"
const jobGroups = Array.from(document.querySelectorAll(".postings-group"));

const engineeringCategory = jobGroups.find(job => job.firstChild.innerHTML === "Product Engineering");

const engJobs = Array.from(engineeringCategory.querySelectorAll(".posting")).map(node => {
     const title = node.querySelector("h5")?.innerText ?? "Unknown Title";
    const link = node.querySelector(".posting-title")?.href ?? "#";
    
    // Use a compound selector to directly select the last <span> inside .posting-categories
    const location = node.querySelector(".posting-categories span:last-child")?.innerHTML ?? "Unknown Location";
    
    return {
        title,
        link,
        location,
     }
})

JSON.stringify(engJobs)

"##,
};

pub const WEEDMAPS_SCRAPE_OPTIONS: DefaultJobScraperOptions = DefaultJobScraperOptions {
    url: "https://boards.greenhouse.io/embed/job_board?for=weedmaps77&b=https%3A%2F%2Fweedmaps.com%2Fcareers",
    headless: false,
    company_key: "Weedmaps",
    content_selector: "body",
    get_jobs_js: r#"
    JSON.stringify(Array.from(document.querySelectorAll('div[department_id="4069853002,4069854002"]')).map(job => {
    const link = job.querySelector("a").href;
    const [title, location] = job.innerText.split("\n")
    return {
        title,
        location,
        link
    }
}))

    "#
};

pub const DISCORD_SCRAPE_OPTIONS: DefaultJobScraperOptions = DefaultJobScraperOptions {
    headless: true,
    url: "https://discord.com/careers",
    company_key: "Discord",
    content_selector: "body",
    get_jobs_js: r#"
    const jobs = [...document.querySelectorAll("div[data-department-name='Product Engineering'], div[data-department-name='IT'], div[data-department-name='Product Design'], div[data-department-name='Data Platform & Data Engineering'], div[data-department-name='Core Tech Engineering'], div[data-department-name='Activities Platform']")].map(el => {
    
const categoryJobs = [...el.querySelectorAll(".card-job")]

return categoryJobs.map(job => ({
    link: job.href,
    title: job.querySelector("h3").innerHTML,
    location: job.querySelector("div").innerHTML,

    
}))

}).flat()

JSON.stringify(jobs);
    "#,
};

pub const GITHUB_SCRAPE_OPTIONS: DefaultJobScraperOptions = DefaultJobScraperOptions {
    headless: true,
    url: "https://www.github.careers/careers-home/jobs?categories=Engineering&page=1&limit=100",
    company_key: "GitHub",
    content_selector: "body",
    get_jobs_js: r#"
const jobsPayload = [...document.querySelectorAll(".mat-content")].map(el => {

    const title = el.querySelector(".job-title").innerText;
     const location = el.querySelector(".location").innerHTML.slice(0, -2);
    const link = el.querySelector("a").href;

    return {
        title,
        location,
        link // href
        
    }
});

JSON.stringify(jobsPayload); 
    "#,
};

pub const GITLAB_SCRAPE_OPTIONS: DefaultJobScraperOptions = DefaultJobScraperOptions {
    url: "https://about.gitlab.com/jobs/all-jobs/#engineering",
    headless: true,
    company_key: "GitLab",
    content_selector: "#engineering",
    get_jobs_js: r##"
    const engSection = document.querySelector("#engineering");

const jobs = [...engSection.querySelectorAll(".job")].map(j => {

    const title = j.querySelector("a").innerText;
    const link = j.querySelector("a").href;
    const location = j.querySelector("p").innerText;

    return {

        title,
        link,
        location
    }
})


JSON.stringify(jobs)
    "##,
};


pub const THE_BROWSER_COMPANY_DEFAULT_SCRAPE_OPTIONS: DefaultJobScraperOptions = DefaultJobScraperOptions {
    url: "https://jobs.ashbyhq.com/The%20Browser%20Company?departmentId=48071c1e-bfb0-4c30-985f-88ab3ce24920",
    headless: true,
    company_key: "The Browser Company",
    content_selector: "._departments_12ylk_345",
    get_jobs_js: r#"
    const jobs_cont = document.querySelector("._departments_12ylk_345")


const jobs = [...jobs_cont.querySelectorAll("[class^=' _container']")].map(j => {

    const title = j.querySelector("h3").innerText;
    const locations = j.querySelector("p").innerText.split("•");

    
    const location = locations[1] + locations[2]

    return {
        link: j.href,
        title,
        location
    }
})

JSON.stringify(jobs)
    "#
};


pub const PALANTIR_DEFAULT_SCRAPE_OPTIONS: DefaultJobScraperOptions = DefaultJobScraperOptions {
    url: "https://jobs.lever.co/palantir/",
    headless:  true,
    company_key: "Palantir",
    content_selector: "body",
    get_jobs_js: r#"
    const postingsGroups = document.querySelectorAll('.postings-group');

// Use .find to locate the matching group
const matchingGroup = Array.from(postingsGroups).find(group => {
  const firstChild = group.firstElementChild;

  return (
    firstChild &&
    firstChild.classList.contains('posting-category-title') &&
    firstChild.classList.contains('large-category-label') &&
    firstChild.textContent.trim() === 'Dev'
  );
});


const jobs = [...matchingGroup.querySelectorAll(".posting-title")].map(j => {

    return {
        link: j.href,
        title: j.querySelector("h5").textContent,
        location: j.querySelector("span.location").textContent
    }
})

JSON.stringify(jobs)


    "#
};
