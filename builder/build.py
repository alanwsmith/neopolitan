#!/usr/bin/env python3

import os

from datetime import datetime
from string import Template


class Builder():
    def __init__(self):
        pass

        # self.source_root = f"{self.project_root}/builder/src"
        # self.site_root = f"{self.project_root}/site"
        # self.parts = {}

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
        template = Template(self._template)
        with open(path, 'w') as _out:
            _out.write(
                template.substitute({
                    "BODY": "This is the stuff",
                    "PRE": 'werwerwer'
                })
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
    b.load_template(f"{project_root}/builder/templates/home-page.html")
    b.load_content(f"{project_root}/builder/content/reference.neo")
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
