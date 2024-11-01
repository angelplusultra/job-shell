const jobs = [...document.querySelectorAll(".JobListing_jobListItems__lXfbo")].map(item => {
    const link = item.querySelector("a").href;
    const pTags = [...item.querySelectorAll("p")];
    return {
        title: pTags[0].innerText,
        location: pTags[1].innerText,
        link
    }
    
})

JSON.stringify(jobs);
