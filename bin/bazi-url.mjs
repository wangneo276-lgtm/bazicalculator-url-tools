#!/usr/bin/env node
import { homeUrl, pageUrl } from "../index.js";
const key = process.argv[2] || "home";
process.stdout.write((key === "home" ? homeUrl() : pageUrl(key)) + "\n");
