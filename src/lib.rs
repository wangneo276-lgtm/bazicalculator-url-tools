const ORIGIN: &str = "https://bazicalculator.pro";
const HOST: &str = "bazicalculator.pro";
const DESCRIPTION: &str = "Free BaZi calculator: Four Pillars, Day Master, Ten Gods, Five Elements and Luck Pillars with true solar time. Plus Zi Wei Dou Shu, Liu Yao, Mei Hua, Qi Men and the Chinese Almanac.";

const PATHS: &[&str] = &[
    "/",
    "/about",
    "/blog",
    "/blog/bazi-compatibility-basics",
    "/blog/category/bazi",
    "/blog/category/bazi/page/2",
    "/blog/category/bazi/page/3",
    "/blog/category/method",
    "/blog/category/tools",
    "/blog/chinese-almanac-yi-ji",
    "/blog/combinations-and-clashes-in-bazi",
    "/blog/companions-friend-and-rob-wealth",
    "/blog/day-master",
    "/blog/day-master-strength",
    "/blog/daylight-saving-and-bazi",
    "/blog/earthly-branches-and-hidden-stems",
    "/blog/five-elements-in-bazi",
    "/blog/heavenly-stems-explained",
    "/blog/li-chun-and-the-bazi-year",
    "/blog/liu-yao-coin-method",
    "/blog/luck-pillars",
    "/blog/mei-hua-yi-shu-time-casting",
    "/blog/page/2",
    "/blog/page/3",
    "/blog/page/4",
    "/blog/page/5",
    "/blog/pillar-positions-in-bazi",
    "/blog/qi-men-dun-jia-basics",
    "/blog/reading-annual-pillars",
    "/blog/solar-terms-and-bazi-months",
    "/blog/ten-gods",
    "/blog/ten-gods-output-and-resource",
    "/blog/ten-gods-wealth-and-officer",
    "/blog/true-solar-time",
    "/blog/twelve-growth-stages",
    "/blog/unknown-birth-time-bazi",
    "/blog/useful-element-yong-shen",
    "/blog/what-is-bazi",
    "/blog/zi-hour-day-boundary",
    "/blog/zi-wei-dou-shu-basics",
    "/changelog",
    "/chinese-almanac",
    "/compare",
    "/compare/bazi-vs-western-astrology",
    "/compare/bazi-vs-zi-wei-dou-shu",
    "/compare/liu-yao-vs-mei-hua",
    "/compare/qi-men-vs-liu-yao",
    "/compare/true-solar-time-vs-clock-time",
    "/compare/zi-hour-23-vs-00",
    "/compatibility",
    "/compatibility/earth-and-earth",
    "/compatibility/earth-and-metal",
    "/compatibility/earth-and-water",
    "/compatibility/fire-and-earth",
    "/compatibility/fire-and-fire",
    "/compatibility/fire-and-metal",
    "/compatibility/fire-and-water",
    "/compatibility/metal-and-metal",
    "/compatibility/metal-and-water",
    "/compatibility/water-and-water",
    "/compatibility/wood-and-earth",
    "/compatibility/wood-and-fire",
    "/compatibility/wood-and-metal",
    "/compatibility/wood-and-water",
    "/compatibility/wood-and-wood",
    "/day-master",
    "/day-master/bing-fire",
    "/day-master/ding-fire",
    "/day-master/geng-metal",
    "/day-master/gui-water",
    "/day-master/ji-earth",
    "/day-master/jia-wood",
    "/day-master/ren-water",
    "/day-master/wu-earth",
    "/day-master/xin-metal",
    "/day-master/yi-wood",
    "/earthly-branches",
    "/earthly-branches/chen",
    "/earthly-branches/chou",
    "/earthly-branches/hai",
    "/earthly-branches/mao",
    "/earthly-branches/shen",
    "/earthly-branches/si",
    "/earthly-branches/wei",
    "/earthly-branches/wu",
    "/earthly-branches/xu",
    "/earthly-branches/yin",
    "/earthly-branches/you",
    "/earthly-branches/zi",
    "/email-support",
    "/faq",
    "/faq/bazi",
    "/faq/chinese-almanac",
    "/faq/liu-yao",
    "/faq/mei-hua-yi-shu",
    "/faq/qi-men-dun-jia",
    "/faq/zi-wei-dou-shu",
    "/five-elements",
    "/five-elements/earth",
    "/five-elements/earth-strong",
    "/five-elements/earth-weak",
    "/five-elements/fire",
    "/five-elements/fire-strong",
    "/five-elements/fire-weak",
    "/five-elements/metal",
    "/five-elements/metal-strong",
    "/five-elements/metal-weak",
    "/five-elements/water",
    "/five-elements/water-strong",
    "/five-elements/water-weak",
    "/five-elements/wood",
    "/five-elements/wood-strong",
    "/five-elements/wood-weak",
    "/glossary",
    "/glossary/annual-pillar",
    "/glossary/body-and-use",
    "/glossary/branch-clash",
    "/glossary/branch-harm",
    "/glossary/branch-punishment",
    "/glossary/changing-line",
    "/glossary/chart-structure",
    "/glossary/chinese-new-year-vs-li-chun",
    "/glossary/controlling-cycle",
    "/glossary/day-master",
    "/glossary/day-pillar",
    "/glossary/directional-combination",
    "/glossary/earthly-branches",
    "/glossary/favorable-element",
    "/glossary/five-elements",
    "/glossary/follower-chart",
    "/glossary/four-pillars",
    "/glossary/generating-cycle",
    "/glossary/heavenly-stems",
    "/glossary/hexagram",
    "/glossary/hidden-stems",
    "/glossary/hour-pillar",
    "/glossary/luck-direction-term",
    "/glossary/luck-pillars",
    "/glossary/month-pillar",
    "/glossary/na-jia",
    "/glossary/na-yin",
    "/glossary/nine-palaces",
    "/glossary/nobleman-star",
    "/glossary/peach-blossom",
    "/glossary/root",
    "/glossary/seasonal-strength",
    "/glossary/sexagenary-cycle",
    "/glossary/six-combinations",
    "/glossary/solar-terms",
    "/glossary/start-of-spring",
    "/glossary/starting-age",
    "/glossary/stem-clash",
    "/glossary/stem-combination",
    "/glossary/strong-day-master",
    "/glossary/ten-gods",
    "/glossary/three-harmony",
    "/glossary/travelling-horse",
    "/glossary/trigram",
    "/glossary/true-solar-time",
    "/glossary/twelve-growth-stages",
    "/glossary/unfavorable-element",
    "/glossary/useful-element",
    "/glossary/void-emptiness",
    "/glossary/weak-day-master",
    "/glossary/year-pillar",
    "/glossary/yin-yang",
    "/glossary/zi-hour",
    "/heavenly-stems",
    "/heavenly-stems/bing",
    "/heavenly-stems/ding",
    "/heavenly-stems/geng",
    "/heavenly-stems/gui",
    "/heavenly-stems/ji",
    "/heavenly-stems/jia",
    "/heavenly-stems/ren",
    "/heavenly-stems/wu",
    "/heavenly-stems/xin",
    "/heavenly-stems/yi",
    "/learn",
    "/learn/check-your-bazi-chart",
    "/learn/combinations-and-clashes",
    "/learn/find-the-useful-element",
    "/learn/find-your-day-master",
    "/learn/four-pillars-of-destiny",
    "/learn/how-to-read-a-bazi-chart",
    "/learn/judge-day-master-strength",
    "/learn/read-luck-pillars-step-by-step",
    "/learn/read-ten-gods-in-context",
    "/learn/unknown-birth-time",
    "/liu-yao",
    "/luck-pillars",
    "/luck-pillars/annual-pillar",
    "/luck-pillars/changing-luck-pillars",
    "/luck-pillars/how-to-read-a-luck-pillar",
    "/luck-pillars/luck-and-natal-interactions",
    "/luck-pillars/luck-direction",
    "/luck-pillars/starting-age",
    "/mei-hua-yi-shu",
    "/methodology",
    "/pricing",
    "/privacy-policy",
    "/qi-men-dun-jia",
    "/refund-policy",
    "/ten-gods",
    "/ten-gods/direct-officer",
    "/ten-gods/direct-resource",
    "/ten-gods/direct-wealth",
    "/ten-gods/eating-god",
    "/ten-gods/friend",
    "/ten-gods/hurting-officer",
    "/ten-gods/indirect-resource",
    "/ten-gods/indirect-wealth",
    "/ten-gods/rob-wealth",
    "/ten-gods/seven-killings",
    "/terms-of-service",
    "/tools",
    "/tools/annual-pillar-calculator",
    "/tools/bazi-compatibility-calculator",
    "/tools/chinese-calendar-converter",
    "/tools/day-master-calculator",
    "/tools/five-elements-calculator",
    "/tools/hidden-stems-calculator",
    "/tools/luck-pillar-calculator",
    "/tools/ten-gods-calculator",
    "/tools/true-solar-time-calculator",
    "/zh",
    "/zh/about",
    "/zh/blog",
    "/zh/blog/bazi-compatibility-basics",
    "/zh/blog/category/bazi",
    "/zh/blog/category/bazi/page/2",
    "/zh/blog/category/bazi/page/3",
    "/zh/blog/category/method",
    "/zh/blog/category/tools",
    "/zh/blog/chinese-almanac-yi-ji",
    "/zh/blog/combinations-and-clashes-in-bazi",
    "/zh/blog/companions-friend-and-rob-wealth",
    "/zh/blog/day-master",
    "/zh/blog/day-master-strength",
    "/zh/blog/daylight-saving-and-bazi",
    "/zh/blog/earthly-branches-and-hidden-stems",
    "/zh/blog/five-elements-in-bazi",
    "/zh/blog/heavenly-stems-explained",
    "/zh/blog/li-chun-and-the-bazi-year",
    "/zh/blog/liu-yao-coin-method",
    "/zh/blog/luck-pillars",
    "/zh/blog/mei-hua-yi-shu-time-casting",
    "/zh/blog/page/2",
    "/zh/blog/page/3",
    "/zh/blog/page/4",
    "/zh/blog/page/5",
    "/zh/blog/pillar-positions-in-bazi",
    "/zh/blog/qi-men-dun-jia-basics",
    "/zh/blog/reading-annual-pillars",
    "/zh/blog/solar-terms-and-bazi-months",
    "/zh/blog/ten-gods",
    "/zh/blog/ten-gods-output-and-resource",
    "/zh/blog/ten-gods-wealth-and-officer",
    "/zh/blog/true-solar-time",
    "/zh/blog/twelve-growth-stages",
    "/zh/blog/unknown-birth-time-bazi",
    "/zh/blog/useful-element-yong-shen",
    "/zh/blog/what-is-bazi",
    "/zh/blog/zi-hour-day-boundary",
    "/zh/blog/zi-wei-dou-shu-basics",
    "/zh/changelog",
    "/zh/chinese-almanac",
    "/zh/compare",
    "/zh/compare/bazi-vs-western-astrology",
    "/zh/compare/bazi-vs-zi-wei-dou-shu",
    "/zh/compare/liu-yao-vs-mei-hua",
    "/zh/compare/qi-men-vs-liu-yao",
    "/zh/compare/true-solar-time-vs-clock-time",
    "/zh/compare/zi-hour-23-vs-00",
    "/zh/compatibility",
    "/zh/compatibility/earth-and-earth",
    "/zh/compatibility/earth-and-metal",
    "/zh/compatibility/earth-and-water",
    "/zh/compatibility/fire-and-earth",
    "/zh/compatibility/fire-and-fire",
    "/zh/compatibility/fire-and-metal",
    "/zh/compatibility/fire-and-water",
    "/zh/compatibility/metal-and-metal",
    "/zh/compatibility/metal-and-water",
    "/zh/compatibility/water-and-water",
    "/zh/compatibility/wood-and-earth",
    "/zh/compatibility/wood-and-fire",
    "/zh/compatibility/wood-and-metal",
    "/zh/compatibility/wood-and-water",
    "/zh/compatibility/wood-and-wood",
    "/zh/day-master",
    "/zh/day-master/bing-fire",
    "/zh/day-master/ding-fire",
    "/zh/day-master/geng-metal",
    "/zh/day-master/gui-water",
    "/zh/day-master/ji-earth",
    "/zh/day-master/jia-wood",
    "/zh/day-master/ren-water",
    "/zh/day-master/wu-earth",
    "/zh/day-master/xin-metal",
    "/zh/day-master/yi-wood",
    "/zh/earthly-branches",
    "/zh/earthly-branches/chen",
    "/zh/earthly-branches/chou",
    "/zh/earthly-branches/hai",
    "/zh/earthly-branches/mao",
    "/zh/earthly-branches/shen",
    "/zh/earthly-branches/si",
    "/zh/earthly-branches/wei",
    "/zh/earthly-branches/wu",
    "/zh/earthly-branches/xu",
    "/zh/earthly-branches/yin",
    "/zh/earthly-branches/you",
    "/zh/earthly-branches/zi",
    "/zh/faq",
    "/zh/faq/bazi",
    "/zh/faq/chinese-almanac",
    "/zh/faq/liu-yao",
    "/zh/faq/mei-hua-yi-shu",
    "/zh/faq/qi-men-dun-jia",
    "/zh/faq/zi-wei-dou-shu",
    "/zh/five-elements",
    "/zh/five-elements/earth",
    "/zh/five-elements/earth-strong",
    "/zh/five-elements/earth-weak",
    "/zh/five-elements/fire",
    "/zh/five-elements/fire-strong",
    "/zh/five-elements/fire-weak",
    "/zh/five-elements/metal",
    "/zh/five-elements/metal-strong",
    "/zh/five-elements/metal-weak",
    "/zh/five-elements/water",
    "/zh/five-elements/water-strong",
    "/zh/five-elements/water-weak",
    "/zh/five-elements/wood",
    "/zh/five-elements/wood-strong",
    "/zh/five-elements/wood-weak",
    "/zh/glossary",
    "/zh/glossary/annual-pillar",
    "/zh/glossary/body-and-use",
    "/zh/glossary/branch-clash",
    "/zh/glossary/branch-harm",
    "/zh/glossary/branch-punishment",
    "/zh/glossary/changing-line",
    "/zh/glossary/chart-structure",
    "/zh/glossary/chinese-new-year-vs-li-chun",
    "/zh/glossary/controlling-cycle",
    "/zh/glossary/day-master",
    "/zh/glossary/day-pillar",
    "/zh/glossary/directional-combination",
    "/zh/glossary/earthly-branches",
    "/zh/glossary/favorable-element",
    "/zh/glossary/five-elements",
    "/zh/glossary/follower-chart",
    "/zh/glossary/four-pillars",
    "/zh/glossary/generating-cycle",
    "/zh/glossary/heavenly-stems",
    "/zh/glossary/hexagram",
    "/zh/glossary/hidden-stems",
    "/zh/glossary/hour-pillar",
    "/zh/glossary/luck-direction-term",
    "/zh/glossary/luck-pillars",
    "/zh/glossary/month-pillar",
    "/zh/glossary/na-jia",
    "/zh/glossary/na-yin",
    "/zh/glossary/nine-palaces",
    "/zh/glossary/nobleman-star",
    "/zh/glossary/peach-blossom",
    "/zh/glossary/root",
    "/zh/glossary/seasonal-strength",
    "/zh/glossary/sexagenary-cycle",
    "/zh/glossary/six-combinations",
    "/zh/glossary/solar-terms",
    "/zh/glossary/start-of-spring",
    "/zh/glossary/starting-age",
    "/zh/glossary/stem-clash",
    "/zh/glossary/stem-combination",
    "/zh/glossary/strong-day-master",
    "/zh/glossary/ten-gods",
    "/zh/glossary/three-harmony",
    "/zh/glossary/travelling-horse",
    "/zh/glossary/trigram",
    "/zh/glossary/true-solar-time",
    "/zh/glossary/twelve-growth-stages",
    "/zh/glossary/unfavorable-element",
    "/zh/glossary/useful-element",
    "/zh/glossary/void-emptiness",
    "/zh/glossary/weak-day-master",
    "/zh/glossary/year-pillar",
    "/zh/glossary/yin-yang",
    "/zh/glossary/zi-hour",
    "/zh/heavenly-stems",
    "/zh/heavenly-stems/bing",
    "/zh/heavenly-stems/ding",
    "/zh/heavenly-stems/geng",
    "/zh/heavenly-stems/gui",
    "/zh/heavenly-stems/ji",
    "/zh/heavenly-stems/jia",
    "/zh/heavenly-stems/ren",
    "/zh/heavenly-stems/wu",
    "/zh/heavenly-stems/xin",
    "/zh/heavenly-stems/yi",
    "/zh/learn",
    "/zh/learn/check-your-bazi-chart",
    "/zh/learn/combinations-and-clashes",
    "/zh/learn/find-the-useful-element",
    "/zh/learn/find-your-day-master",
    "/zh/learn/four-pillars-of-destiny",
    "/zh/learn/how-to-read-a-bazi-chart",
    "/zh/learn/judge-day-master-strength",
    "/zh/learn/read-luck-pillars-step-by-step",
    "/zh/learn/read-ten-gods-in-context",
    "/zh/learn/unknown-birth-time",
    "/zh/liu-yao",
    "/zh/luck-pillars",
    "/zh/luck-pillars/annual-pillar",
    "/zh/luck-pillars/changing-luck-pillars",
    "/zh/luck-pillars/how-to-read-a-luck-pillar",
    "/zh/luck-pillars/luck-and-natal-interactions",
    "/zh/luck-pillars/luck-direction",
    "/zh/luck-pillars/starting-age",
    "/zh/mei-hua-yi-shu",
    "/zh/methodology",
    "/zh/pricing",
    "/zh/qi-men-dun-jia",
    "/zh/ten-gods",
    "/zh/ten-gods/direct-officer",
    "/zh/ten-gods/direct-resource",
    "/zh/ten-gods/direct-wealth",
    "/zh/ten-gods/eating-god",
    "/zh/ten-gods/friend",
    "/zh/ten-gods/hurting-officer",
    "/zh/ten-gods/indirect-resource",
    "/zh/ten-gods/indirect-wealth",
    "/zh/ten-gods/rob-wealth",
    "/zh/ten-gods/seven-killings",
    "/zh/tools",
    "/zh/tools/annual-pillar-calculator",
    "/zh/tools/bazi-compatibility-calculator",
    "/zh/tools/chinese-calendar-converter",
    "/zh/tools/day-master-calculator",
    "/zh/tools/five-elements-calculator",
    "/zh/tools/hidden-stems-calculator",
    "/zh/tools/luck-pillar-calculator",
    "/zh/tools/ten-gods-calculator",
    "/zh/tools/true-solar-time-calculator",
    "/zh/zi-wei-dou-shu-calculator",
    "/zi-wei-dou-shu-calculator"
];

fn known(path: &str) -> bool {
    PATHS.binary_search(&path).is_ok()
}

fn english_only(path: &str) -> bool {
    matches!(path, "/email-support" | "/privacy-policy" | "/refund-policy" | "/terms-of-service")
}

pub fn description() -> &'static str {
    DESCRIPTION
}

pub fn canonical_url(input: &str) -> Result<String, &'static str> {
    let raw = input.trim();
    if raw.is_empty() {
        return Err("path required");
    }
    if raw.contains('\\') || raw.contains('%') {
        return Err("invalid path");
    }
    let raw = raw.split('#').next().unwrap_or(raw);
    let raw = raw.split('?').next().unwrap_or(raw);
    let lower = raw.to_ascii_lowercase();
    let path = if lower.starts_with("https://") || lower.starts_with("http://") || raw.starts_with("//") {
        let rest = if lower.starts_with("https://") {
            &raw["https://".len()..]
        } else if lower.starts_with("http://") {
            &raw["http://".len()..]
        } else {
            &raw[2..]
        };
        let (hostport, path) = match rest.find('/') {
            Some(i) => (&rest[..i], &rest[i..]),
            None => (rest, "/"),
        };
        if hostport.contains('@') || hostport.contains(':') {
            return Err("host not allowed");
        }
        if !hostport.eq_ignore_ascii_case(HOST) {
            return Err("host not allowed");
        }
        path.to_string()
    } else if raw.starts_with('/') {
        raw.to_string()
    } else {
        format!("/{raw}")
    };
    let mut path = path.to_ascii_lowercase();
    while path.contains("//") {
        path = path.replace("//", "/");
    }
    if path.len() > 1 {
        path = path.trim_end_matches('/').to_string();
    }
    if path.is_empty() {
        path = "/".to_string();
    }
    if !known(&path) {
        return Err("unknown path");
    }
    if path == "/" {
        Ok(format!("{ORIGIN}/"))
    } else {
        Ok(format!("{ORIGIN}{path}"))
    }
}

fn page_path(key: &str) -> Option<&'static str> {
    match key {
        "about" => Some("/about"),
        "changelog" => Some("/changelog"),
        "chinese-almanac" => Some("/chinese-almanac"),
        "compare" => Some("/compare"),
        "compatibility" => Some("/compatibility"),
        "day-master" => Some("/day-master"),
        "earthly-branches" => Some("/earthly-branches"),
        "faq" => Some("/faq"),
        "five-elements" => Some("/five-elements"),
        "glossary" => Some("/glossary"),
        "heavenly-stems" => Some("/heavenly-stems"),
        "home" => Some("/"),
        "learn" => Some("/learn"),
        "liu-yao" => Some("/liu-yao"),
        "luck-pillars" => Some("/luck-pillars"),
        "mei-hua-yi-shu" => Some("/mei-hua-yi-shu"),
        "methodology" => Some("/methodology"),
        "pricing" => Some("/pricing"),
        "qi-men-dun-jia" => Some("/qi-men-dun-jia"),
        "ten-gods" => Some("/ten-gods"),
        "tools" => Some("/tools"),
        "zi-wei-dou-shu-calculator" => Some("/zi-wei-dou-shu-calculator"),
        _ => None,
    }
}

fn blog_path(slug: &str) -> Option<&'static str> {
    match slug {
        "bazi-compatibility-basics" => Some("/blog/bazi-compatibility-basics"),
        "chinese-almanac-yi-ji" => Some("/blog/chinese-almanac-yi-ji"),
        "combinations-and-clashes-in-bazi" => Some("/blog/combinations-and-clashes-in-bazi"),
        "companions-friend-and-rob-wealth" => Some("/blog/companions-friend-and-rob-wealth"),
        "day-master" => Some("/blog/day-master"),
        "day-master-strength" => Some("/blog/day-master-strength"),
        "daylight-saving-and-bazi" => Some("/blog/daylight-saving-and-bazi"),
        "earthly-branches-and-hidden-stems" => Some("/blog/earthly-branches-and-hidden-stems"),
        "five-elements-in-bazi" => Some("/blog/five-elements-in-bazi"),
        "heavenly-stems-explained" => Some("/blog/heavenly-stems-explained"),
        "li-chun-and-the-bazi-year" => Some("/blog/li-chun-and-the-bazi-year"),
        "liu-yao-coin-method" => Some("/blog/liu-yao-coin-method"),
        "luck-pillars" => Some("/blog/luck-pillars"),
        "mei-hua-yi-shu-time-casting" => Some("/blog/mei-hua-yi-shu-time-casting"),
        "pillar-positions-in-bazi" => Some("/blog/pillar-positions-in-bazi"),
        "qi-men-dun-jia-basics" => Some("/blog/qi-men-dun-jia-basics"),
        "reading-annual-pillars" => Some("/blog/reading-annual-pillars"),
        "solar-terms-and-bazi-months" => Some("/blog/solar-terms-and-bazi-months"),
        "ten-gods" => Some("/blog/ten-gods"),
        "ten-gods-output-and-resource" => Some("/blog/ten-gods-output-and-resource"),
        "ten-gods-wealth-and-officer" => Some("/blog/ten-gods-wealth-and-officer"),
        "true-solar-time" => Some("/blog/true-solar-time"),
        "twelve-growth-stages" => Some("/blog/twelve-growth-stages"),
        "unknown-birth-time-bazi" => Some("/blog/unknown-birth-time-bazi"),
        "useful-element-yong-shen" => Some("/blog/useful-element-yong-shen"),
        "what-is-bazi" => Some("/blog/what-is-bazi"),
        "zi-hour-day-boundary" => Some("/blog/zi-hour-day-boundary"),
        "zi-wei-dou-shu-basics" => Some("/blog/zi-wei-dou-shu-basics"),
        _ => None,
    }
}

pub fn home_url() -> String {
    canonical_url("/").expect("home")
}

pub fn page_url(key: &str) -> Result<String, &'static str> {
    let path = page_path(key).ok_or("unknown page")?;
    canonical_url(path)
}

pub fn blog_url(slug: &str) -> Result<String, &'static str> {
    let path = blog_path(slug).ok_or("unknown blog")?;
    canonical_url(path)
}

pub fn locale_url(locale: &str, key: &str) -> Result<String, &'static str> {
    if locale != "en" && locale != "zh" {
        return Err("locale not available");
    }
    let path = page_path(key).ok_or("unknown page")?;
    if locale == "zh" && english_only(path) {
        return Err("locale not available");
    }
    if locale == "en" {
        canonical_url(path)
    } else {
        let prefixed = if path == "/" { "/zh".to_string() } else { format!("/zh{path}") };
        canonical_url(&prefixed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_and_about() {
        assert_eq!(home_url(), "https://bazicalculator.pro/");
        assert_eq!(page_url("about").unwrap(), "https://bazicalculator.pro/about");
        assert_eq!(blog_url("bazi-compatibility-basics").unwrap(), "https://bazicalculator.pro/blog/bazi-compatibility-basics");
        assert_eq!(locale_url("zh", "about").unwrap(), "https://bazicalculator.pro/zh/about");
        assert!(locale_url("zh", "privacy").is_err() || locale_url("zh", "privacy-policy").is_err());
        assert!(canonical_url("https://example.com/").is_err());
        assert!(description().contains("BaZi"));
    }
}
