const express = require("express");

const app = express();
const port = 8080;

app.use(
  express.static("./dist", {
    setHeaders: (res) => {
      res.setHeader("Cross-Origin-Embedder-Policy", "require-corp");
      res.setHeader("Cross-Origin-Opener-Policy", "same-origin");
    },
  })
);

app.listen(port, () => console.log(`Listening at http://localhost:${port}`));
