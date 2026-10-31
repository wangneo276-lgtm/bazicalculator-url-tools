# bazicalculator-url-tools

Builds canonical HTTPS URLs for public pages on [BaZi Calculator](https://bazicalculator.pro/).

Free BaZi calculator: Four Pillars, Day Master, Ten Gods, Five Elements and Luck Pillars with true solar time. Plus Zi Wei Dou Shu, Liu Yao, Mei Hua, Qi Men and the Chinese Almanac.

Not an official SDK. The helper only accepts paths that are already on the public site, including English and the Chinese (`/zh`) translations. Privacy, terms, refund, and email support stay English-only. It does not calculate a chart or call the site.

## Examples

- `homeUrl()` → https://bazicalculator.pro/
- `pageUrl("about")` → https://bazicalculator.pro/about
- `localeUrl("zh", "about")` → https://bazicalculator.pro/zh/about
- `blogUrl("bazi-compatibility-basics")` → https://bazicalculator.pro/blog/bazi-compatibility-basics

Support: support@bazicalculator.pro

Source: https://github.com/wangneo276-lgtm/bazicalculator-url-tools

License: MIT.
