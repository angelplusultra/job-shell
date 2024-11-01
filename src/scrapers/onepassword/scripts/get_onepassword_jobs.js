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

