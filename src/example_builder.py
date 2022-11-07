#!/usr/bin/env python3

import os

from datetime import datetime
from string import Template
from neopolitan import Neopolitan
from html import escape

class Builder():
    def __init__(self):
        self._sections = {}

    def load_template(self, path):
        with open(path) as _template:
            self._template = _template.read()

    def load_content(self, path):
        with open(path) as _content:
            self._content = _content.read()

    # def build_content(self):
    #     # Make dynamic content here
    #     # self.parts['CONTENT'] = "the quick brown fox"
    #     pass

    # def load_parts(self):
    #     for file_part in self.file_parts:
    #         with open(f"{self.source_root}/{file_part}") as _file_part:
    #             name_parts = file_part.split('.')
    #             self.parts[name_parts[0]] = _file_part.read()

    def output_page(self, path):

        with open("_example_content/intro.neo") as _neo_i:
            content = _neo_i.read()
            n = Neopolitan()
            n.load(content)
            self._sections['intro_html'] = n.content()
            self._sections['intro_html_text'] = escape(n.content())
            self._sections['intro_neo'] = escape(content)


        with open("_example_content/links.neo") as _neo_i:
            content = _neo_i.read()
            n = Neopolitan()
            n.load(content)
            self._sections['links_html'] = n.content()
            self._sections['links_html_text'] = escape(n.content())
            self._sections['links_neo'] = escape(content)


        template = Template(self._template)
        with open(path, 'w') as _out:
            _out.write(
                template.substitute(
                    self._sections
                )
            )

    # def make_page(self, template_path, output_path, data):
    #     with open(template_path) as _template:
    #         template = Template(_template.read())
    #         with open(output_path, 'w') as _output:
    #             _output.write(
    #                 template.substitute(data)
    #             )

if __name__ == "__main__":
    project_root = os.path.dirname(os.path.dirname(__file__))
    b = Builder()
    b.load_template(f"{project_root}/src/_example_content/_template.html")

    # b.load_content(f"{project_root}/src/_example_content/intro.neo")
    b.output_page(f"{project_root}/site/index.html")




    # b.load_basic_post('lib/')

    # b.file_parts = []
    # b.load_parts()
    # b.build_content()
    # b.make_page(
    #     f"{b.source_root}/TEMPLATE.html",
    #     f"{b.site_root}/index.html",
    #     b.parts
    # )

    print(f"## Completed Build: {datetime.now()}")
