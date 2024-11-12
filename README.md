
---
<div align="center"><img src="https://cdn.discordapp.com/attachments/917180495849197568/1305854030899314688/jobshell_icon.png?ex=67348ad6&is=67333956&hm=50424aa0d51b9d09890e781b528cb0d98988835f3be280f390f93f12cc6715ff&" alt="Description of the image" width="250" height="250">
</div>


# JobShell

JobShell is a command-line tool designed to streamline your software engineering job search by delivering relevant job listings directly to your terminal. With a curated selection of companies—regularly updated—you can view the latest job openings from companies you're interested in without the hassle of visiting multiple job boards. JobShell’s intuitive interface lets you browse job roles and get AI-generated insights, such as responsibilities, requirements, and essential details for each role. When you find an opportunity that stands out, apply directly from the terminal or continue exploring—all within one streamlined experience.

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
     GEMINI_KEY=your_gemini_api_key_here
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
- Reddit
- Weedmaps
  
## How the Program Works

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

```

When adding a new company to the repo, the first step is adding a new company to the `COMPANYKEYS` global array variable in `main.rs`.
