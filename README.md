
---

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

You can get your Gemini API Key from Google’s [Gemini API Portal](https://cloud.google.com/gemini).

## Installation

1. **Clone the Repository**
   ```bash
   git clone https://github.com/yourusername/jobshell.git
   cd jobshell
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

---

### Note
Make sure Chrome and Node.js are correctly installed and up-to-date. Contact support if you encounter issues setting up JobShell.

Happy job hunting with JobShell—bringing your job search to the terminal!
