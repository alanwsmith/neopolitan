#!/usr/bin/env python3

import glob
import os

from string import Template

single_shorthands = [
        { "NAME": "tilde", "TOKEN": "~"},
        { "NAME": "backtick", "TOKEN": "`"},
        { "NAME": "ampersand", "TOKEN": "@"},
        { "NAME": "caret", "TOKEN": "^"},
        { "NAME": "astrisk", "TOKEN": "*"},
        { "NAME": "underscore", "TOKEN": "_"},
        { "NAME": "open_paren", "TOKEN": "("},
        { "NAME": "close_paren", "TOKEN": ")"},
        { "NAME": "open_square", "TOKEN": "["},
        { "NAME": "close_square", "TOKEN": "]"},
        { "NAME": "open_curly", "TOKEN": "{"},
        { "NAME": "close_curly", "TOKEN": "}"},
        { "NAME": "open_tag", "TOKEN": "<"},
        { "NAME": "close_tag", "TOKEN": ">"},
        { "NAME": "pipe", "TOKEN": "|"},
        { "NAME": "dash", "TOKEN": "-"},
        { "NAME": "dash", "TOKEN": "1"},
        { "NAME": "dash", "TOKEN": "2"},
        { "NAME": "dash", "TOKEN": "3"},
        { "NAME": "dash", "TOKEN": "4"},
        { "NAME": "dash", "TOKEN": "5"},
        { "NAME": "dash", "TOKEN": "6"},
        { "NAME": "dash", "TOKEN": "7"},
        { "NAME": "dash", "TOKEN": "8"},
        { "NAME": "dash", "TOKEN": "9"},
        { "NAME": "dash", "TOKEN": "0"},
        { "NAME": "dash", "TOKEN": "#"},
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



