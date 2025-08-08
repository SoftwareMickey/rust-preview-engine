const express = require("express");
const cors = require("cors");
const morgan = require("morgan");

const app = express();

// * Middleware
app.use(cors({
  origin: "*",
  methods: ["POST", "PATCH", "PUT", "GET", "DELETE"],
  allowedHeaders: ["Content-Type", "Authorization"],
}));

app.use(morgan("combined"));
app.use(express.json());


module.exports = { app };
