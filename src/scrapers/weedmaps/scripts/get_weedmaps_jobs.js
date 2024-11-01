JSON.stringify(Array.from(document.querySelectorAll('div[department_id="4069853002,4069854002"]')).map(job => {
    const link = job.querySelector("a").href;
    const [title, location] = job.innerText.split("\n")
    return {
        title,
        location,
        link
    }
}))
