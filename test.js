import test from "node:test";
import assert from "node:assert/strict";
import { blogUrl, canonicalUrl, homeUrl, localeUrl, metadata, pageUrl } from "./index.js";

test("home, page, locale, and blog", () => {
  assert.equal(homeUrl(), "https://bazicalculator.pro/");
  assert.equal(pageUrl("about"), "https://bazicalculator.pro/about");
  assert.equal(blogUrl("bazi-compatibility-basics"), "https://bazicalculator.pro/blog/bazi-compatibility-basics");
  assert.equal(canonicalUrl("https://bazicalculator.pro/zh/about?x=1#y"), "https://bazicalculator.pro/zh/about");
  assert.equal(localeUrl("en", "home"), "https://bazicalculator.pro/");
  assert.equal(localeUrl("zh", "/"), "https://bazicalculator.pro/zh");
  assert.equal(metadata().homepage, "https://bazicalculator.pro/");
  assert.equal(metadata().support, "support@bazicalculator.pro");
  assert.match(metadata().description, /Four Pillars/);
});

test("rejects unknown hosts and untranslated legal pages", () => {
  assert.throws(() => canonicalUrl("/not-a-real-page"), /unknown path/);
  assert.throws(() => canonicalUrl("https://example.com/"), /host not allowed/);
  assert.throws(() => localeUrl("zh", "/privacy-policy"), /locale not available/);
  assert.throws(() => pageUrl("nope"), /unknown page/);
});
