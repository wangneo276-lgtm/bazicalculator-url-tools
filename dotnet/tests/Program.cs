using Bazicalculator.UrlTools;
if (Urls.HomeUrl() != "https://bazicalculator.pro/") return 1;
if (Urls.PageUrl("about") != "https://bazicalculator.pro/about") return 1;
if (Urls.BlogUrl("bazi-compatibility-basics") != "https://bazicalculator.pro/blog/bazi-compatibility-basics") return 1;
if (Urls.LocaleUrl("zh", "about") != "https://bazicalculator.pro/zh/about") return 1;
try { Urls.CanonicalUrl("https://example.com/"); return 1; } catch (ArgumentException) {}
Console.WriteLine("dotnet ok");
return 0;
