

# JobShell: Because Job Hunting is Hell Enough
![GitHub Release](https://img.shields.io/github/v/release/angelplusultra/job-shell?style=flat)
![GitHub Repo stars](https://img.shields.io/github/stars/angelplusultra/job-shell)
![Downloads](https://img.shields.io/github/downloads/angelplusultra/job-shell/total?style=flat)
![Issues](https://img.shields.io/github/issues/angelplusultra/job-shell?style=flat)
![Pull Requests](https://img.shields.io/github/issues-pr/angelplusultra/job-shell?style=flat)
![Crates.io Total Downloads](https://img.shields.io/crates/d/jobshell?label=crates.io%20downloads)
[![Homebrew](https://img.shields.io/badge/homebrew-v1.0.9-blue)](https://github.com/angelplusultra/homebrew-jobshell)
[![crates.io](https://img.shields.io/crates/v/jobshell)](https://crates.io/crates/jobshell)






<!--![GitHub Stars](https://img.shields.io/github/stars/angelplusultra/job-shell?style=social)
![GitHub Forks](https://img.shields.io/github/forks/angelplusultra/job-shell?style=social)
![GitHub Watchers](https://img.shields.io/github/watchers/angelplusultra/job-shell?style=social)-->

Are you a software engineer desperately seeking employment but **done** with LinkedIn’s circus of virtue-signaling posts and irrelevant job alerts? Tired of getting emails that scream “Exciting opportunities in your network!” only to find out John liked Dave’s post about on-site synergy?

Wish you could manage your job search from the comfort of your **terminal cave**, where corporate nonsense can’t reach you? Well, my friend, welcome to JobShell—the no-bullshit solution to staying updated on opportunities at companies you actually care about.

Say goodbye to distractions and hello to streamlined job hunting. 

<!--**JobShell** is a command-line tool designed to streamline your software engineering job hunt by scraping job postings from a curated list of top tech companies. It provides two primary modes of operation—an interactive CLI mode and a Discord integration mode—offering flexibility to suit your workflow. With JobShell, you can manage your network connections, discover new job postings, discover new job postings from the companies YOU care about, and even use AI to do some cool shit here and there.-->
## Table of Contents
- [Key Features](#key-features)
- [Supported Companies](#supported-companies)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Setting Up AI Integration (Optional)](#setting-up-ai-integration-optional)
- [Usage](#usage)
- [Suggested Workflow](#suggested-workflow)
---

## Key Features 

1. **CLI Mode**  
   - **Interactive Navigation:** Run `jobshell` for a terminal-based menu. Scrape jobs from individual companies, view new postings, and manage your professional network from a single interface.
   - **Network-Based Discovery:** Scan for new roles exclusively at companies where you have existing connections or have "followed".
   - **Bookmarks & Draft Messages:** Bookmark interesting jobs for later review and draft personalized opening messages to your connections.

2. **Discord Integration Mode**  
   - **Automated Updates via Webhook:** Use `jobshell --discord` to run a continuous background scrape for all supported companies. It will periodically post new job updates to a specified Discord channel.
   - **Scheduled Execution:** Configure the scraper to run at regular intervals (every 1–12 hours) to stay informed with the latest openings.
<svg fill="#FFFFFF" xmlns="http://www.w3.org/2000/svg" height="200" width="200" viewBox="-72.03675 -32.46875 624.3185 194.8125">
<path d="M92.497 55.588H52.141a1.887 1.887 0 00-1.886 1.887v19.73c0 1.042.845 1.888 1.886 1.888h15.743v24.515s-3.535 1.204-13.308 1.204c-11.53 0-27.636-4.212-27.636-39.63 0-35.426 16.772-40.087 32.517-40.087 13.63 0 19.502 2.4 23.238 3.556 1.174.358 2.26-.81 2.26-1.851l4.502-19.064c0-.488-.165-1.075-.72-1.473C87.22 5.18 77.963 0 54.576 0 27.636 0 0 11.463 0 66.563c0 55.101 31.64 63.312 58.303 63.312 22.076 0 35.468-9.434 35.468-9.434.552-.304.612-1.076.612-1.429V57.475a1.886 1.886 0 00-1.886-1.887M300.475 6.602a1.88 1.88 0 00-1.873-1.897h-22.723a1.889 1.889 0 00-1.881 1.897l.005 43.914h-35.418V6.602c0-1.05-.836-1.897-1.876-1.897h-22.722a1.888 1.888 0 00-1.876 1.897v118.904c0 1.048.843 1.902 1.876 1.902h22.722c1.04 0 1.876-.854 1.876-1.902v-50.86h35.418l-.061 50.86c0 1.048.841 1.902 1.883 1.902H298.6c1.04 0 1.872-.854 1.874-1.902zM135.376 22.205c0-8.181-6.56-14.793-14.653-14.793-8.085 0-14.65 6.612-14.65 14.793 0 8.174 6.565 14.804 14.65 14.804 8.093 0 14.653-6.63 14.653-14.804m-1.625 78.219V45.537c0-1.041-.84-1.893-1.88-1.893h-22.65c-1.04 0-1.97 1.07-1.97 2.113v78.636c0 2.31 1.44 2.998 3.304 2.998h20.408c2.239 0 2.788-1.1 2.788-3.035zm253.081-56.602h-22.548c-1.035 0-1.876.852-1.876 1.902v58.301s-5.73 4.192-13.86 4.192c-8.13 0-10.288-3.69-10.288-11.65V45.723c0-1.05-.84-1.902-1.875-1.902H313.5c-1.032 0-1.879.852-1.879 1.902v54.692c0 23.646 13.179 29.432 31.308 29.432 14.875 0 26.867-8.218 26.867-8.218s.57 4.331.83 4.844c.257.512.93 1.03 1.658 1.03l14.559-.064c1.032 0 1.878-.854 1.878-1.899l-.008-79.817c0-1.05-.842-1.902-1.881-1.902m52.736 64.324c-7.822-.239-13.127-3.787-13.127-3.787V66.703s5.233-3.208 11.655-3.781c8.12-.727 15.944 1.725 15.944 21.096 0 20.425-3.53 24.457-14.472 24.127m8.893-66.994c-12.807 0-21.517 5.715-21.517 5.715V6.602c0-1.05-.84-1.897-1.875-1.897h-22.788a1.887 1.887 0 00-1.877 1.897v118.904c0 1.05.841 1.903 1.88 1.903h15.81c.712 0 1.251-.368 1.65-1.011.393-.639.96-5.481.96-5.481s9.317 8.829 26.956 8.829c20.71 0 32.585-10.504 32.585-47.155 0-36.65-18.968-41.44-31.784-41.44m-249.403 2.482h-17.045l-.026-22.519c0-.852-.438-1.278-1.425-1.278h-23.227c-.902 0-1.388.398-1.388 1.266v23.27s-11.64 2.809-12.426 3.037a1.886 1.886 0 00-1.362 1.812v14.623c0 1.05.84 1.9 1.879 1.9h11.91v35.178c0 26.128 18.327 28.695 30.694 28.695 5.652 0 12.412-1.815 13.528-2.227.675-.248 1.068-.946 1.068-1.704l.019-16.086c0-1.05-.887-1.9-1.884-1.9-.994 0-3.535.405-6.151.405-8.372 0-11.21-3.892-11.21-8.93V65.743h17.046a1.89 1.89 0 001.881-1.9V45.528c0-1.05-.842-1.895-1.881-1.895" fill="#FFFFFF"/>
</svg>


---

## Supported Companies
  
<table>
  <tr>
    <!-- 1Password -->
    <td>
      <img 
        alt="1Password" 
        width="150" 
        height="150" 
        src="https://upload.wikimedia.org/wikipedia/commons/0/02/1Password_wordmark_blue_2023.svg" 
      />
    </td>
    <!-- Airbnb -->
    <td>
      <img 
        alt="Airbnb" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idkuvXnjOH/theme/dark/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
    <!-- Blizzard -->
    <td>
      <img 
        alt="Blizzard" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idEyJgSpdD/theme/dark/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
    <!-- Chase -->
    <td>
      <img 
        alt="Chase" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idudVYts5w/theme/light/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
  </tr>
  <tr>
    <!-- Cisco -->
    <td>
      <img 
        alt="Cisco" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/ida_xaMYlM/id7nCQRNp4.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
    <!-- Cloudflare -->
    <td>
      <img 
        alt="Cloudflare" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idJ3Cg8ymG/idASSo3XVu.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
    <!-- Coinbase -->
    <td>
      <img 
        alt="Coinbase" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idwDWo4ONQ/theme/light/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
    <!-- CoStar -->
    <td>
      <img 
        alt="CoStar" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idvP5FAI8W/theme/light/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
  </tr>
  <tr>
    <!-- Discord -->
    <td>
      <img 
        alt="Discord" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idM8Hlme1a/theme/dark/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
    <!-- Disney -->
    <td>
      <img 
        alt="Disney" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idxASqzkm_/theme/light/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
    <!-- Experian -->
    <td>
      <img 
        alt="Experian" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idRdK0bkoQ/theme/dark/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
    <!-- GitHub -->
    <td>
      <img 
        alt="GitHub" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idZAyF9rlg/theme/light/symbol.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
  </tr>
  <tr>
    <!-- GitLab -->
    <td>
      <img 
        alt="GitLab" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idw382nG0m/theme/dark/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
    <!-- IBM -->
    <td>
      <img 
        alt="IBM" 
        width="150" 
        height="150" 
        src="https://upload.wikimedia.org/wikipedia/commons/5/51/IBM_logo.svg" 
      />
    </td>
    <!-- Meta -->
    <td>
      <img 
        alt="Meta" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idWvz5T3V7/theme/light/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
    <!-- Netflix -->
    <td>
      <img 
        alt="Netflix" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/ideQwN5lBE/theme/dark/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
  </tr>
  <tr>
    <!-- Nike -->
    <td>
      <img 
        alt="Nike" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/id_0dwKPKT/theme/light/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
    <!-- Norton -->
    <td>
      <img 
        alt="Norton" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idPZ21fCdZ/theme/light/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
    <!-- Palantir -->
    <td>
      <img 
        alt="Palantir" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/id4EZOUw_e/theme/light/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
    <!-- Reddit -->
    <td>
      <img 
        alt="Reddit" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idkKwm0IT0/theme/dark/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
  </tr>
  <tr>
    <!-- Robinhood -->
    <td>
      <img 
        alt="Robinhood" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/id3WzK3p17/theme/light/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
    <!-- Salesforce -->
    <td>
      <img 
        alt="Salesforce" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idVE84WdIN/theme/dark/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
    <!-- ServiceNow -->
    <td>
      <img 
        alt="ServiceNow" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idgONjBNKe/theme/light/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
    <!-- Square -->
    <td>
      <img 
        alt="Square" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idyxZCX73R/theme/light/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
  </tr>
  <tr>
    <!-- Stripe -->
    <td>
      <img 
        alt="Stripe" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idxAg10C0L/theme/dark/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
<!-- Toast -->
   <td>
      <img 
        alt="Toast" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/idJCbAfjvP/theme/dark/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
 <!-- Uber -->
   <td>
      <img 
        alt="Uber" 
        width="150" 
        height="150" 
        src="https://cdn.brandfetch.io/ididNbiiOd/theme/light/logo.svg?c=1dxbfHSJFAPEGdCLU4o5B" 
      />
    </td>
  </tr>
</table>



 **[See Full List of Companies](https://github.com/angelplusultra/job-shell/wiki/Supported-Companies)**

### Want a company added to JobShell?  
**[Submit a Company Request](https://github.com/angelplusultra/job-shell/issues/new?assignees=&labels=&projects=&template=company-request.md&title=Company+Request%3A+%7BCOMPANY_NAME%7D)** by opening a GitHub issue.


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
## Installation
### 🏠 Install via Homebrew (macOS)

```bash
brew tap angelplusultra/jobshell
brew install jobshell
```
### 📦 Install via Cargo (All Platforms)

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

## Verify Installation
```
jobshell --version
```
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

### ⌨ CLI Mode

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
     Scan all companies? (otherwise only followed companies or companies where you have at least 1 connection) (yes/no)
     ```
Once all prompts all completed, JobShell begins scraping job postings at the specified hourly interval and new job postings will be sent to the provided Discord channel webhook.

---

## Suggested Workflow

To get the most out of JobShell, follow this workflow:
	
1.	Set Up Your Preferences in CLI Mode
   	- Start by running JobShell in CLI mode. Configure your connections and specify the companies you want to track for job opportunities.
2.	Deploy JobShell in Discord Mode
   	- Once set up, switch to Discord mode to receive real-time job notifications directly in your designated Discord channel.
3.	Run JobShell on a VPS (Optional)
	- If you don’t have a machine to keep JobShell running continuously, consider using a Virtual Private Server (VPS) from providers like DigitalOcean, Linode, or Vultr.
---

That’s it! You’re all set to simplify your job search with JobShell.
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
