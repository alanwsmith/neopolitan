#!/usr/bin/env python3

import glob
import os

from string import Template

single_shorthands = [
        { "NAME": "tilde", "TOKEN": "~"},
        { "NAME": "backtick", "TOKEN": "`"},
        ]

template_dir = "templates"
output_dir = "../../tests/ok"

file_names = [
    os.path.basename(file) for file in glob.glob(f"{template_dir}/*")
    if os.path.isfile(file)
]

for shorthand in single_shorthands:
    for file_name in file_names:
        with open(f"{template_dir}/{file_name}", "r") as _tmpl:
            output_path = f"{output_dir}/generated-{shorthand["NAME"]}-{file_name}"
            print(output_path)
            template = Template(_tmpl.read())
            output = template.substitute(shorthand)
            with open(output_path, "w") as _out:
                _out.write(output)


