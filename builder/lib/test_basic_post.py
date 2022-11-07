#!/usr/bin/env python3

import unittest

from basic_post import BasicPost

class TestBasicPost(unittest.TestCase):

    def setUp(self):
        global bp 
        bp = BasicPost()

        global alpha_sample_source
        with open("_data/alpha_sample_source.org") as _alpha_source:
            alpha_sample_source = _alpha_source.read()

        global alpha_sample_target_html
        with open("_data/alpha_sample_target_html.txt") as _alpha_target:
            alpha_sample_target_html = _alpha_target.read()

        global alpha_sample_target_html_lines
        alpha_sample_target_html_lines = alpha_sample_target_html.split("\n")



        self.maxDiff = None 

    def test_integration(self):
        bp.load_file(alpha_sample_source)
        with open('../../site/index.html', 'w') as _debug:
            _debug.write(bp.body())

    def test_output(self):
        lines = 2
        bp.load_file(alpha_sample_source)
        output_lines = bp.body().split("\n")
        for line_num in range(0, lines):
            expected = alpha_sample_target_html_lines[line_num]
            result = output_lines[line_num]
            self.assertEqual(result, expected, f'LINE: {line_num + 1}')


if __name__ == '__main__':
    unittest.main()

