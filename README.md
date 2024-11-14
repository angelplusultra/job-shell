
---
<div align="center"><img src="https://cdn.discordapp.com/attachments/917180495849197568/1305854030899314688/jobshell_icon.png?ex=67348ad6&is=67333956&hm=50424aa0d51b9d09890e781b528cb0d98988835f3be280f390f93f12cc6715ff&" alt="Description of the image" width="250" height="250">
</div>


# JobShell

JobShell is a command-line tool designed to streamline your software engineering job search by delivering relevant job listings directly to your terminal. With a curated selection of companies—regularly updated—you can view the latest job openings from companies you're interested in without the hassle of visiting multiple job boards. JobShell’s intuitive interface lets you browse job roles and get AI-generated insights, such as responsibilities, requirements, and essential details for each role. When you find an opportunity that stands out, apply directly from the terminal or continue exploring—all within one streamlined experience.

## Demo
<div align="left">
   <img src="assets/jobshell_demo_2.gif" />
</div>


<!-- ## Features -->
<!-- - **Curated Company Selection**: Choose from a regularly updated list of companies, ensuring you’re always seeing the freshest and most relevant openings. -->
<!-- - **AI-Enhanced Insights**: JobShell uses AI to analyze job listings, providing detailed insights on each role, including responsibilities, requirements, and skills. -->
<!-- - **Direct Application**: Apply to roles directly from your terminal, helping you streamline the job application process. -->

## Prerequisites
To run JobShell, make sure you have:
- **Google Chrome** installed
- **Node.js v20+**
- **Google Gemini API Key**

You can get your Gemini API Key from Google’s [Gemini API Portal](https://ai.google.dev/).

## Installation

1. **Clone the Repository**
   ```bash
   git clone https://github.com/angelplusultra/job-shell.git
   cd job-shell
   ```

2. **Install Dependencies**  
   Ensure Node.js and Chrome are installed. Install any additional dependencies as needed.

3. **Set Up Environment Variables**  
   - Create a `.env` file in the root of the repository:
     ```bash
     touch .env
     ```
   - Open `.env` in a text editor and add the following line:
     ```plaintext
     GEMINI_KEY={YOUR_GEMINI_API_KEY}
     GEMINI_MODEL=flash 
     ```
   - Replace `your_gemini_api_key_here` with your actual Gemini API key.

4. **Run JobShell**  
   To start JobShell, run:
   ```bash
   cargo run
   ```

## Usage
Once JobShell is running:
1. **Select a Company**  
   You’ll be prompted to choose from a list of curated companies. New companies are added regularly.

2. **Explore Job Listings**  
   JobShell fetches the latest job openings for the selected company and displays them in your terminal.

3. **View AI-Generated Insights**  
   For each job listing, JobShell provides AI-driven insights, breaking down responsibilities, requirements, and other essential details.

4. **Apply or Continue Browsing**  
   When you’re ready, you can apply to roles or continue exploring other opportunities.
   
## Companies Currently Supported
- 1Password
- Anduril
- Discord
- GitHub
- GitLab
- Reddit
- The Browser Company
- Weedmaps
  
## How JobShell Works

### Data

All data including scraped jobs and the connections you create are stored in a local `data.json` file at the root level of the repo. 

Example: 
```json
{
  "data": {
    "1Password": {
      "connections": [],
      "jobs": [
        {
          "applied": false,
          "link": "https://jobs.lever.co/1password/e8ed6e04-3455-498a-85df-368a699f7b26",
          "location": "Remote (US or Canada)",
          "title": "Backend Developer (Rust)"
        },
        {
          "applied": false,
          "link": "https://jobs.lever.co/1password/10576c68-8f6e-48d4-9f34-359d95774f6d",
          "location": "Remote (US or Canada)",
          "title": "Engineering Manager, Product Engineering"
        },
        {
          "applied": false,
          "link": "https://jobs.lever.co/1password/5aa629cf-270b-476b-9735-40bc28a1ba49",
          "location": "Remote (US or Canada)",
          "title": "Senior Developer (Backend)"
        },
        {
          "applied": false,
          "link": "https://jobs.lever.co/1password/f6eb3494-6cc8-4658-b861-895f2d33b448",
          "location": "Remote (US or Canada)",
          "title": "Senior Developer, Product Engineering"
        },
        {
          "applied": false,
          "link": "https://jobs.lever.co/1password/67e64bcb-23d7-43b2-b36f-09f231996922",
          "location": "Remote (US or Canada)",
          "title": "Senior Manager, Engineering"
        },
        {
          "applied": false,
          "link": "https://jobs.lever.co/1password/9a581a3f-2604-4496-bfb1-63fab7afa0be",
          "location": "Remote (US or Canada)",
          "title": "Windows Developer (Rust)"
        }
      ]
    },
}
```

When adding a new company to the repo, the first step is adding a new company to the `COMPANYKEYS` global array variable in `main.rs`.
https://github.com/angelplusultra/job-shell/blob/353c970f446f97142684a67a842df6072d67e8ae/src/main.rs#L22-L29

When you run JobShell it will check the `COMPANYKEYS` against the saved state of data, if there is a new company key added it creates the company entry in the `data.json`.

### Creating a New Job Scraper

There are currently 2 options to creating a new scraper

 #### 1. Using the `default_job_scraper` function

If the careers site of the new company you'd like to add is simple, and doesnt require much DOM interaction to retrieve all the jobs, the `default_job_scraper` function might be enough.

https://github.com/angelplusultra/job-shell/blob/ab480388519279665c5dce1bec60116506aa2dbb/src/handlers/handlers.rs#L12-L45

You would just need to create the `DefaultJobScraperOptions` struct instance for your new company in `./src/handlers/scrape_options.rs`

Example: 

```rust
pub const GITHUB_SCRAPE_OPTIONS: DefaultJobScraperOptions = DefaultJobScraperOptions {
   // Whether or not chrome should launch in headless mode
    headless: true,
   // The careers page URL (Can include query params to filter jobs)
    url: "https://www.github.careers/careers-home/jobs?categories=Engineering&page=1&limit=100",
   // The "key" in the data hash map, should be the same name as what you added in `COMPANYKEYS`
    company_key: "GitHub",
   // The selector on the page with the content you're trying to scrape
    content_selector: "body",

   /*
The JavaScript string for returning an array of

{
   title: string,
   location: string,
   link: string // The apply href
}
*/
    get_jobs_js: r#"
const jobsPayload = [...document.querySelectorAll(".mat-content")].map(el => {

    const title = el.querySelector(".job-title").innerText;
     const location = el.querySelector(".location").innerHTML.slice(0, -2);
    const link = el.querySelector("a").href;

    return {
        title,
        location,
        link
        
    }
});

JSON.stringify(jobsPayload); 
    "#,
};

```
Here is a syntax highlighted example of the JavaScript needed for a deafult scraper. It MUST end with a JSON stringified array of:

```ts
{
   title: string,
   location: string,
   link: string // the apply href
}
```

Full Example:
```js
const jobs = [...document.querySelectorAll(".mat-content")].map(el => {

    const title = el.querySelector(".job-title").innerText;
     const location = el.querySelector(".location").innerHTML.slice(0, -2);
    const link = el.querySelector("a").href;

    return {
        title,
        location,
        link
        
    }
});

JSON.stringify(jobs); 
```
### DOCS CURRENTLY IN DEVELOPMENT
