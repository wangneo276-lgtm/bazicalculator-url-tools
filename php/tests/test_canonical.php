<?php
require __DIR__ . '/../src/Bazicalculator/UrlTools/Canonical.php';
use Bazicalculator\UrlTools\Canonical;
if (Canonical::homeUrl() !== 'https://bazicalculator.pro/') exit(1);
if (Canonical::pageUrl('about') !== 'https://bazicalculator.pro/about') exit(1);
if (Canonical::blogUrl('bazi-compatibility-basics') !== 'https://bazicalculator.pro/blog/bazi-compatibility-basics') exit(1);
if (Canonical::localeUrl('zh', 'about') !== 'https://bazicalculator.pro/zh/about') exit(1);
try { Canonical::canonicalUrl('https://example.com/'); exit(1); } catch (InvalidArgumentException $e) {}
echo "php ok\n";
