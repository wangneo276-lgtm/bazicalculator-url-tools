require_relative 'lib/bazicalculator_url_tools'
raise 'home' unless BazicalculatorUrlTools.home_url == 'https://bazicalculator.pro/'
raise 'about' unless BazicalculatorUrlTools.page_url('about') == 'https://bazicalculator.pro/about'
raise 'blog' unless BazicalculatorUrlTools.blog_url('bazi-compatibility-basics') == 'https://bazicalculator.pro/blog/bazi-compatibility-basics'
raise 'zh' unless BazicalculatorUrlTools.locale_url('zh', 'about') == 'https://bazicalculator.pro/zh/about'
begin
  BazicalculatorUrlTools.canonical_url('https://example.com/')
  raise 'host should fail'
rescue ArgumentError
end
puts 'ok'
