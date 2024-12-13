

# JobShell: Because Job Hunting is Hell Enough
![GitHub Release](https://img.shields.io/github/v/release/angelplusultra/job-shell?style=flat)
![Downloads](https://img.shields.io/github/downloads/angelplusultra/job-shell/total?style=flat)
![Issues](https://img.shields.io/github/issues/angelplusultra/job-shell?style=flat)
![Pull Requests](https://img.shields.io/github/issues-pr/angelplusultra/job-shell?style=flat)
![Crates.io Total Downloads](https://img.shields.io/crates/d/jobshell?label=crates.io%20downloads)
![Crates.io Size](https://img.shields.io/crates/size/jobshell)





<!--![GitHub Stars](https://img.shields.io/github/stars/angelplusultra/job-shell?style=social)
![GitHub Forks](https://img.shields.io/github/forks/angelplusultra/job-shell?style=social)
![GitHub Watchers](https://img.shields.io/github/watchers/angelplusultra/job-shell?style=social)-->

Are you a software engineer desperately seeking employment but done with LinkedIn’s circus of virtue-signaling posts and irrelevant job alerts? Tired of getting emails that scream “Exciting opportunities in your network!” only to find out John liked Dave’s post about on-site synergy?

Wish you could manage your job search from the comfort of your **terminal cave**, where corporate nonsense can’t reach you? Well, my friend, welcome to JobShell—the no-bullshit solution to staying updated on opportunities at companies you actually care about.

Say goodbye to distractions and hello to streamlined job hunting. 

<!--**JobShell** is a command-line tool designed to streamline your software engineering job hunt by scraping job postings from a curated list of top tech companies. It provides two primary modes of operation—an interactive CLI mode and a Discord integration mode—offering flexibility to suit your workflow. With JobShell, you can manage your network connections, discover new job postings, discover new job postings from the companies YOU care about, and even use AI to do some cool shit here and there.-->

---

## 🗝️ Key Features 

1. **CLI Mode**  
   - **Interactive Navigation:** Run `jobshell` for a terminal-based menu. Scrape jobs from individual companies, view new postings, and manage your professional network from a single interface.
   - **Network-Based Discovery:** Scan for new roles exclusively at companies where you have existing connections or have "followed".
   - **Bookmarks & Draft Messages:** Bookmark interesting jobs for later review and draft personalized opening messages to your connections.
   - **AI Integration (Experimental):** Optional integration with Gemini AI for generating tailored outreach messages.

2. **Discord Integration Mode**  
   - **Automated Updates via Webhook:** Use `jobshell --discord` to run a continuous background scrape for all supported companies. It will periodically post new job updates to a specified Discord channel.
   - **Scheduled Execution:** Configure the scraper to run at regular intervals (every 1–12 hours) to stay informed with the latest openings.

---

## 🏢 Supported Companies (22)

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

## 📋 Prerequisites

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
## 💻 Installation
### 📦 Install via Cargo

```bash
cargo install jobshell
```

### 📥 Install from Release (macOS and Windows only for now)
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
   
### 🛠️ Build from Source

1. **Install Rust:**  
   If not already installed on your system, [Install Rust](https://www.rust-lang.org/tools/install).

2. **Clone the Repo:**  
   Clone the repo to your system and `cd` into it

    ```
   git clone https://github.com/angelplusultra/job-shell && cd job-shell
   ```

3. **Create a Release Build:**  
   Compile the JobShell binary for your platform
   ```
   cargo build --release
   ```

4. **Add the Binary to Your `$PATH`:**  
   Move or copy the compiled binary to a directory on your `$PATH`.
   
   An example: 
   ```
   mv ./target/release/jobshell /usr/bin/jobshell 
   ```
---

## ✨ Setting Up AI Integration (Optional)

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

## 🚀 Usage

### ⌨️ CLI Mode

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

### 🤖 Discord Mode

```
jobshell --discord
```
When `jobshell --discord` is executed, a wizard will guide you through the setup process. This wizard collects the necessary information to configure the job-scraping process and ensures Discord notifications are set up correctly.

#### Wizard Steps

1. **Enter Discord Channel Webhook URL**
   - You’ll be prompted to provide the webhook URL for the Discord channel where job notifications will be posted.
   - Example: https://discord.com/api/webhooks/someid/someid
        - **How to Get a Discord Webhook URL**
           1. Open your Discord server
           2. Navigate to the desired channel and click the gear icon to open the channel settings.
           3. Go to the Integrations tab.
           4. Select Webhooks and click Create Webhook.
           5. Customize the webhook settings, copy the Webhook URL, and paste it into the prompt.
2. **Set Hourly Interval**
   - Enter the scraping interval in hours. This value must be an integer between 1 and 12.
   - Example Input: `6`
3. **Choose Scraping Scope**
   - Specify whether you want to scrape all supported companies or restrict scraping to:
        - Companies you have at least one connection with.
        - Companies you’ve chosen to “follow” via CLI mode.
   - Prompt Example:

     ```
     Would you like to scrape all supported companies? (yes/no)
     ```
Once all prompts all completed, JobShell begins scraping job postings at the specified hourly interval and new job postings will be sent to the provided Discord channel webhook.

---
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
