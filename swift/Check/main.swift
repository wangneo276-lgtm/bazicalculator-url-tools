import BazicalculatorUrlTools
guard BazicalculatorUrlTools.homeUrl() == "https://bazicalculator.pro/" else { fatalError("home") }
guard (try BazicalculatorUrlTools.pageUrl("about")) == "https://bazicalculator.pro/about" else { fatalError("about") }
guard (try BazicalculatorUrlTools.blogUrl("bazi-compatibility-basics")) == "https://bazicalculator.pro/blog/bazi-compatibility-basics" else { fatalError("blog") }
guard (try BazicalculatorUrlTools.localeUrl("zh", "about")) == "https://bazicalculator.pro/zh/about" else { fatalError("zh") }
print("swift ok")
