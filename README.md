<!---->
<!-- --- -->
<!-- <div align="center"><img src="https://cdn.discordapp.com/attachments/917180495849197568/1305854030899314688/jobshell_icon.png?ex=67348ad6&is=67333956&hm=50424aa0d51b9d09890e781b528cb0d98988835f3be280f390f93f12cc6715ff&" alt="Description of the image" width="250" height="250"> -->
<!-- </div> -->
<!---->
<!---->
<!-- # JobShell -->
<!---->
<!-- JobShell is a command-line tool designed to streamline your software engineering job search by delivering relevant job listings directly to your terminal. With a curated selection of companies—regularly updated—you can view the latest job openings from companies you're interested in without the hassle of visiting multiple job boards. JobShell’s intuitive interface lets you browse job roles and get AI-generated insights, such as responsibilities, requirements, and essential details for each role. When you find an opportunity that stands out, apply directly from the terminal or continue exploring—all within one streamlined experience. -->
<!---->
<!-- ## Demo -->
<!-- <div align="left"> -->
<!--    <img src="assets/jobshell_demo_2.gif" /> -->
<!-- </div> -->
<!---->
<!---->
<!-- <!-- ## Features --> -->
<!-- <!-- - **Curated Company Selection**: Choose from a regularly updated list of companies, ensuring you’re always seeing the freshest and most relevant openings. --> -->
<!-- <!-- - **AI-Enhanced Insights**: JobShell uses AI to analyze job listings, providing detailed insights on each role, including responsibilities, requirements, and skills. --> -->
<!-- <!-- - **Direct Application**: Apply to roles directly from your terminal, helping you streamline the job application process. --> -->
<!---->
<!-- ## Prerequisites -->
<!-- To run JobShell, make sure you have: -->
<!-- - **Google Chrome** installed -->
<!-- - **Node.js v20+** -->
<!-- - **Google Gemini API Key** -->
<!---->
<!-- You can get your Gemini API Key from Google’s [Gemini API Portal](https://ai.google.dev/). -->
<!---->
<!-- ## Installation -->
<!---->
<!-- 1. **Clone the Repository** -->
<!--    ```bash -->
<!--    git clone https://github.com/angelplusultra/job-shell.git -->
<!--    cd job-shell -->
<!--    ``` -->
<!---->
<!-- 2. **Install Dependencies**   -->
<!--    Ensure Node.js and Chrome are installed. Install any additional dependencies as needed. -->
<!---->
<!-- 3. **Set Up Environment Variables**   -->
<!--    - Create a `.env` file in the root of the repository: -->
<!--      ```bash -->
<!--      touch .env -->
<!--      ``` -->
<!--    - Open `.env` in a text editor and add the following line: -->
<!--      ```plaintext -->
<!--      GEMINI_KEY={YOUR_GEMINI_API_KEY} -->
<!--      GEMINI_MODEL=flash  -->
<!--      ``` -->
<!--    - Replace `your_gemini_api_key_here` with your actual Gemini API key. -->
<!---->
<!-- 4. **Run JobShell**   -->
<!--    To start JobShell, run: -->
<!--    ```bash -->
<!--    cargo run -->
<!--    ``` -->
<!---->
<!-- ## Usage -->
<!-- Once JobShell is running: -->
<!-- 1. **Select a Company**   -->
<!--    You’ll be prompted to choose from a list of curated companies. New companies are added regularly. -->
<!---->
<!-- 2. **Explore Job Listings**   -->
<!--    JobShell fetches the latest job openings for the selected company and displays them in your terminal. -->
<!---->
<!-- 3. **View AI-Generated Insights**   -->
<!--    For each job listing, JobShell provides AI-driven insights, breaking down responsibilities, requirements, and other essential details. -->
<!---->
<!-- 4. **Apply or Continue Browsing**   -->
<!--    When you’re ready, you can apply to roles or continue exploring other opportunities. -->
<!--     -->
<!-- ## Supported Companies (22) -->
<!-- - 1Password -->
<!-- - Anduril -->
<!-- - Blizzard -->
<!-- - Chase -->
<!-- - Cisco -->
<!-- - Coinbase -->
<!-- - CoStar Group -->
<!-- - Discord -->
<!-- - Experian -->
<!-- - Disney -->
<!-- - Gen -->
<!-- - GitHub -->
<!-- - GitLab -->
<!-- - IBM -->
<!-- - Meta -->
<!-- - Netflix -->
<!-- - Palantir -->
<!-- - Reddit -->
<!-- - Salesforce -->
<!-- - Square -->
<!-- - The Browser Company -->
<!-- - Weedmaps -->
<!--    -->
<!-- ## How JobShell Works -->
<!---->
<!-- ### Data -->
<!---->
<!-- All data including scraped jobs and the connections you create are stored in a local `data.json` file at the root level of the repo.  -->
<!---->
<!-- Example:  -->
<!-- ```json -->
<!-- { -->
<!--   "data": { -->
<!--     "1Password": { -->
<!--       "connections": [], -->
<!--       "jobs": [ -->
<!--         { -->
<!--           "applied": false, -->
<!--           "link": "https://jobs.lever.co/1password/e8ed6e04-3455-498a-85df-368a699f7b26", -->
<!--           "location": "Remote (US or Canada)", -->
<!--           "title": "Backend Developer (Rust)" -->
<!--         }, -->
<!--         { -->
<!--           "applied": false, -->
<!--           "link": "https://jobs.lever.co/1password/10576c68-8f6e-48d4-9f34-359d95774f6d", -->
<!--           "location": "Remote (US or Canada)", -->
<!--           "title": "Engineering Manager, Product Engineering" -->
<!--         }, -->
<!--         { -->
<!--           "applied": false, -->
<!--           "link": "https://jobs.lever.co/1password/5aa629cf-270b-476b-9735-40bc28a1ba49", -->
<!--           "location": "Remote (US or Canada)", -->
<!--           "title": "Senior Developer (Backend)" -->
<!--         }, -->
<!--         { -->
<!--           "applied": false, -->
<!--           "link": "https://jobs.lever.co/1password/f6eb3494-6cc8-4658-b861-895f2d33b448", -->
<!--           "location": "Remote (US or Canada)", -->
<!--           "title": "Senior Developer, Product Engineering" -->
<!--         }, -->
<!--         { -->
<!--           "applied": false, -->
<!--           "link": "https://jobs.lever.co/1password/67e64bcb-23d7-43b2-b36f-09f231996922", -->
<!--           "location": "Remote (US or Canada)", -->
<!--           "title": "Senior Manager, Engineering" -->
<!--         }, -->
<!--         { -->
<!--           "applied": false, -->
<!--           "link": "https://jobs.lever.co/1password/9a581a3f-2604-4496-bfb1-63fab7afa0be", -->
<!--           "location": "Remote (US or Canada)", -->
<!--           "title": "Windows Developer (Rust)" -->
<!--         } -->
<!--       ] -->
<!--     }, -->
<!-- } -->
<!-- ``` -->
<!---->
<!-- When adding a new company to the repo, the first step is adding a new company to the `COMPANYKEYS` global array variable in `main.rs`. -->
<!-- https://github.com/angelplusultra/job-shell/blob/353c970f446f97142684a67a842df6072d67e8ae/src/main.rs#L22-L29 -->
<!---->
<!-- When you run JobShell it will check the `COMPANYKEYS` against the saved state of data, if there is a new company key added it creates the company entry in the `data.json`. -->
<!---->
<!-- ### Creating a New Job Scraper -->
<!---->
<!-- There are currently 2 options to creating a new scraper -->
<!---->
<!--  #### 1. Using the `default_job_scraper` function -->
<!---->
<!-- If the careers site of the new company you'd like to add is simple, and doesnt require much DOM interaction to retrieve all the jobs, the `default_job_scraper` function might be enough. -->
<!---->
<!-- https://github.com/angelplusultra/job-shell/blob/ab480388519279665c5dce1bec60116506aa2dbb/src/handlers/handlers.rs#L12-L45 -->
<!---->
<!-- You would just need to create the `DefaultJobScraperOptions` struct instance for your new company in `./src/handlers/scrape_options.rs` -->
<!---->
<!-- Example:  -->
<!---->
<!-- ```rust -->
<!-- pub const GITHUB_SCRAPE_OPTIONS: DefaultJobScraperOptions = DefaultJobScraperOptions { -->
<!--    // Whether or not chrome should launch in headless mode -->
<!--     headless: true, -->
<!--    // The careers page URL (Can include query params to filter jobs) -->
<!--     url: "https://www.github.careers/careers-home/jobs?categories=Engineering&page=1&limit=100", -->
<!--    // The "key" in the data hash map, should be the same name as what you added in `COMPANYKEYS` -->
<!--     company_key: "GitHub", -->
<!--    // The selector on the page with the content you're trying to scrape -->
<!--     content_selector: "body", -->
<!---->
<!--    /* -->
<!-- The JavaScript string for returning an array of -->
<!---->
<!-- { -->
<!--    title: string, -->
<!--    location: string, -->
<!--    link: string // The apply href -->
<!-- } -->
<!-- */ -->
<!--     get_jobs_js: r#" -->
<!-- const jobsPayload = [...document.querySelectorAll(".mat-content")].map(el => { -->
<!---->
<!--     const title = el.querySelector(".job-title").innerText; -->
<!--      const location = el.querySelector(".location").innerHTML.slice(0, -2); -->
<!--     const link = el.querySelector("a").href; -->
<!---->
<!--     return { -->
<!--         title, -->
<!--         location, -->
<!--         link -->
<!--          -->
<!--     } -->
<!-- }); -->
<!---->
<!-- JSON.stringify(jobsPayload);  -->
<!--     "#, -->
<!-- }; -->
<!---->
<!-- ``` -->
<!-- Here is a syntax highlighted example of the JavaScript needed for a deafult scraper. It MUST end with a JSON stringified array of: -->
<!---->
<!-- ```ts -->
<!-- { -->
<!--    title: string, -->
<!--    location: string, -->
<!--    link: string // the apply href -->
<!-- } -->
<!-- ``` -->
<!---->
<!-- Full Example: -->
<!-- ```js -->
<!-- const jobs = [...document.querySelectorAll(".mat-content")].map(el => { -->
<!---->
<!--     const title = el.querySelector(".job-title").innerText; -->
<!--      const location = el.querySelector(".location").innerHTML.slice(0, -2); -->
<!--     const link = el.querySelector("a").href; -->
<!---->
<!--     return { -->
<!--         title, -->
<!--         location, -->
<!--         link -->
<!--          -->
<!--     } -->
<!-- }); -->
<!---->
<!-- JSON.stringify(jobs);  -->
<!-- ``` -->
<!-- ### DOCS CURRENTLY IN DEVELOPMENT -->



# JobShell

**JobShell** is a command-line tool designed to streamline your software engineering job hunt by scraping job postings from a curated list of top tech companies. It provides two primary modes of operation—an interactive CLI mode and a Discord integration mode—offering flexibility to suit your workflow. With JobShell, you can manage your network connections, discover new job postings, discover new job postings from the companies YOU care about, and even use AI to do some cool shit here and there.

---

## Key Features

1. **CLI Mode**  
   - **Interactive Navigation:** Run `jobshell` for a terminal-based menu. Scrape jobs from individual companies, view new postings, and manage your professional network from a single interface.
   - **Network-Based Discovery:** Scan for new roles exclusively at companies where you have existing connections.
   - **Bookmarks & Draft Messages:** Bookmark interesting jobs for later review and draft personalized opening messages to your connections.
   - **AI Integration (Experimental):** Optional integration with Gemini AI for generating tailored outreach messages.

2. **Discord Integration Mode**  
   - **Automated Updates via Webhook:** Use `jobshell --discord <WEBHOOK_URL> <HOURS>` to run a continuous background scrape for all supported companies. It will periodically post new job updates to a specified Discord channel.
   - **Scheduled Execution:** Configure the scraper to run at regular intervals (every 1–12 hours) to stay informed with the latest openings.

---

## Supported Companies (22)

JobShell currently supports scraping the following companies:

- 1Password
- Anduril
- Blizzard
- Chase
- Cisco
- Coinbase
- CoStar Group
- Discord
- Experian
- Disney
- Gen
- GitHub
- GitLab
- IBM
- Meta
- Netflix
- Palantir
- Reddit
- Salesforce
- Square
- The Browser Company
- Weedmaps

---

## Prerequisites

Before running JobShell, ensure you have the following:

1. **Detectable Chrome Binary:**  
   JobShell uses browser automation to scrape job postings. You must have a working Chrome installation that can be detected by the underlying scraper.  
   - If you're on macOS, ensure you have Google Chrome installed in the standard location.  
   - On Linux, install Google Chrome via your package manager (e.g., `apt`, `yum`) or download from the official site.  
   - On Windows, ensure Chrome is installed in a standard location or is available in your PATH.

2. **Desktop Environment (or Virtual Environment):**  
   Headless scraping may still require a display server. If running on a server, use `Xvfb` or similar tools to simulate a desktop environment.

3. **Optional Gemini API Key (For Experimental AI Features):**  
   If you plan on using the AI-driven message crafting feature, you’ll need a Gemini API key and a selected model type.

---

### Installation from a Pre-Compiled Binary

1. **Download a Binary from Releases:**  
   Head to the [Releases](https://github.com/angelplusultra/job-shell/releases) page and download the latest binary that matches your operating system.

2. **Place the Binary in Your `$PATH`:**  
   Move the binary to a directory that's included in your system’s `$PATH`. On most UNIX-like systems, this could be:  
   ```bash
   mv jobshell /usr/local/bin/
   ```

3. **Set Executable Permissions:**  
   Ensure the binary is executable:  
   ```bash
   chmod +x /usr/local/bin/jobshell
   ```

4. **Bypass Apple Gatekeeper on macOS (If Necessary):**  
   On macOS, you might need to bypass Gatekeeper’s security checks if the binary isn’t signed:  
   ```bash
   sudo xattr -r -d com.apple.quarantine /usr/local/bin/jobshell
   ```
   If prompted by Gatekeeper, you can also open `System Preferences > Security & Privacy` and choose to "Open Anyway" for the jobshell binary.

---

## Setting Up AI Integration (Optional)

If you would like to leverage the experimental Gemini-based AI features, add the following lines to your shell configuration file (`.zshrc` or `.bashrc`):

```bash
export GEMINI_KEY={your-gemini-key}
export GEMINI_MODEL={model-type} # either "flash" or "pro"
```

**Note:**  
- `GEMINI_KEY` is your API key for Gemini’s service.
- `GEMINI_MODEL` specifies the model type. Choose from "flash" or "pro" depending on your subscription or requirement.

After making these changes, run `source ~/.bashrc` or `source ~/.zshrc` to load the new environment variables.

---

## Usage

### CLI Mode

**Basic Command:**  
Run JobShell in interactive CLI mode:
```bash
jobshell
```

**What You Can Do in CLI Mode:**
- **Scrape Individual Companies:**  
  Choose a company from the menu and scrape the latest postings.
  
- **Manage Connections:**
  Create and manage your personal connections at the supported companies

- **Scan for New Network Jobs:**  
  If you’ve configured your connections, scan for new roles at companies where you have at least one connection.

- **View New Jobs Reports:**  
  Open generated new jobs HTML reports for clearer insights
  
- **Bookmark Jobs:**  
  Mark interesting opportunities for future reference.

- **Reach out to connections:**  
Once you discover a job that interests you and have a connection at the company, JobShell lets you craft a personalized message and open your contact’s LinkedIn profile in one go. Your message, along with the job link, is automatically copied to your clipboard, ready to paste and send.
  

<!-- **Example Workflow:** -->
<!-- 1. Run `./jobshell`. -->
<!-- 2. From the main menu, select **"Selet a Company"**. -->
<!-- 3. Pick a company, select **Manage. -->
<!-- 4. Bookmark a listing or generate an outreach message for a connection at that company. -->
<!-- 5. Exit when done. -->

### Discord Mode

**Command Format:**  
```bash
jobshell --discord <WEBHOOK_URL> <HOURS>
```

**Parameters:**
- `WEBHOOK_URL`: The Discord channel webhook URL where new job postings will be posted.
- `HOURS`: The interval (1–12) at which JobShell will scrape all supported companies and post updates.

**What Happens in Discord Mode:**
- JobShell runs indefinitely, scraping at the specified interval.
- Each time new jobs are detected, they’ll be sent to the specified Discord channel.

**Example:**
```bash
jobshell --discord "https://discordapp.com/api/webhooks/1234/abcd" 6
```
This will post new job updates every 6 hours until you cancel the process (e.g., `Ctrl + C`).
<!---->
<!-- --- -->
<!---->
<!-- ## Cron Integration -->
<!---->
<!-- When running in Discord mode, JobShell schedules its scraping tasks internally. If you prefer to handle scheduling yourself (for example, using `cron` on Linux), you can set up a cron job that launches JobShell at your desired interval: -->
<!---->
<!-- ```cron -->
<!-- 0 */6 * * * /path/to/jobshell --discord https://discordapp.com/api/webhooks/1234/abcd 6 >> /var/log/jobshell.log 2>&1 -->
<!-- ``` -->
<!---->
<!-- **Note:** Using JobShell’s built-in scheduling for Discord mode makes external cron configuration optional. -->
<!---->
<!-- --- -->
<!---->
<!-- ## Troubleshooting & Tips -->
<!---->
<!-- - **Chrome Not Detected:**   -->
<!--   Ensure Chrome is installed and accessible in your PATH. On Linux, you may need `xvfb-run` if running on a headless server: -->
<!--   ```bash -->
<!--   xvfb-run ./jobshell -->
<!--   ``` -->
<!---->
<!-- - **AI Features Not Working:**   -->
<!--   Check that you’ve set `GEMINI_KEY` and `GEMINI_MODEL` correctly, and that these environment variables are sourced in your current shell session. -->
<!---->
<!-- - **Discord Webhook Issues:**   -->
<!--   Verify that the webhook URL is correct and that the Discord channel allows incoming messages from webhooks. -->
<!---->
<!-- --- -->
<!---->
<!-- ## Contributing -->
<!---->
<!-- Contributions to JobShell are welcome! Please open an issue or submit a pull request to discuss new features, improvements, or bug fixes. -->
<!---->
<!-- **Guidelines:** -->
<!-- - Follow coding standards and linting rules where applicable. -->
<!-- - Write clear commit messages and pull request descriptions. -->
<!-- - Include test cases or examples to demonstrate new functionality. -->
<!---->
<!-- --- -->
<!---->
<!-- ## License -->
<!---->
<!-- JobShell is released under the [MIT License](LICENSE). Feel free to use and modify it to fit your job-hunting workflow. -->
<!---->
<!-- --- -->
<!---->
<!-- **Happy Hunting!** Use JobShell to stay on top of new job opportunities and streamline your software engineering job search. -->
