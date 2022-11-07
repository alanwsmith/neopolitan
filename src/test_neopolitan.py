#!/usr/bin/env python3

import unittest

from neopolitan import Neopolitan 

class TestBasicPost(unittest.TestCase):

    def setUp(self):
        self.maxDiff = None 

    def test_intro(self):
        n = Neopolitan()
        with open('_example_content/intro.neo') as _source:
            n.load(_source.read())
        with open('_test_targets/intro.html.txt') as _target:
            target_lines = _target.read().split("\n")
            test_lines = n.content().split("\n")
            line_count = len(target_lines)
            for index in range(0, line_count):
                self.assertEqual(target_lines[index], test_lines[index])

    def test_links(self):
        n = Neopolitan()
        with open('_example_content/links.neo') as _source:
            n.load(_source.read())
        with open('_test_targets/links.html.txt') as _target:
            target_lines = _target.read().split("\n")
            test_lines = n.content().split("\n")
            line_count = len(target_lines)
            for index in range(0, line_count):
                self.assertEqual(target_lines[index], test_lines[index])



    # def test_integration(self):
    #     bp.load_file(alpha_sample_source)
    #     with open('../../site/index.html', 'w') as _debug:
    #         _debug.write(bp.body())

    # def test_output(self):
    #     lines = 2
    #     bp.load_file(alpha_sample_source)
    #     output_lines = bp.body().split("\n")
    #     for line_num in range(0, lines):
    #         expected = alpha_sample_target_html_lines[line_num]
    #         result = output_lines[line_num]
    #         self.assertEqual(result, expected, f'LINE: {line_num + 1}')


if __name__ == '__main__':
    unittest.main()

