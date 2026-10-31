package bazicalculator

import "testing"

func TestHome(t *testing.T) {
    if HomeURL() != "https://bazicalculator.pro/" {
        t.Fatal(HomeURL())
    }
    u, err := PageURL("about")
    if err != nil || u != "https://bazicalculator.pro/about" {
        t.Fatal(u, err)
    }
    b, err := BlogURL("bazi-compatibility-basics")
    if err != nil || b != "https://bazicalculator.pro/blog/bazi-compatibility-basics" {
        t.Fatal(b, err)
    }
    z, err := LocaleURL("zh", "about")
    if err != nil || z != "https://bazicalculator.pro/zh/about" {
        t.Fatal(z, err)
    }
    if _, err := CanonicalURL("https://example.com/"); err == nil {
        t.Fatal("expected host error")
    }
}
