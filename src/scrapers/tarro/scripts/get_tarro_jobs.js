const jobsCont = document.querySelector("#departments-container");

JSON.stringify(
  Array.from(jobsCont.querySelectorAll(".jobs-item")).map((item) => {
    const title = item.querySelector(".job-title")?.innerText;
    const location = item.querySelector(".jobs-info").innerText;

    const link = item.querySelector("a").href;

    return {
      title,
      location,
      link,
    };
  }),
);
