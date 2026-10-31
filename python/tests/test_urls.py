
import unittest
from bazicalculator_url_tools import home_url, canonical_url

class Urls(unittest.TestCase):
    def test_home(self):
        self.assertEqual(home_url(), "https://bazicalculator.pro/")
        self.assertEqual(canonical_url("/about"), "https://bazicalculator.pro/about")

if __name__ == "__main__":
    unittest.main()
