import 'package:bazicalculator_url_tools/bazicalculator_url_tools.dart';
void main() {
  if (homeUrl() != 'https://bazicalculator.pro/') throw StateError('home');
  if (pageUrl('about') != 'https://bazicalculator.pro/about') throw StateError('about');
  if (blogUrl('bazi-compatibility-basics') != 'https://bazicalculator.pro/blog/bazi-compatibility-basics') throw StateError('blog');
  if (localeUrl('zh', 'about') != 'https://bazicalculator.pro/zh/about') throw StateError('zh');
  try { canonicalUrl('https://example.com/'); throw StateError('host'); } on ArgumentError {}
  print('dart ok');
}
