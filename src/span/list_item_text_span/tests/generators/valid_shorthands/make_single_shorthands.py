#!/usr/bin/env python3

import glob
import os

from string import Template

single_shorthands = [
        { "NAME": "backtick", "TOKEN": "`"}
        ]

template_dir = "single_shorthands"
output_dir = "../../tests/valid"

file_names = [
    os.path.basename(file).split(".")[0] for file in glob.glob(f"{template_dir}/*")
    if os.path.isfile(file)
]

for shorthand in single_shorthands:
    for file_name in file_names:
        with open(f"{template_dir}/{file_name}.tmpl", "r") as _tmpl:
            output_path = f"{output_dir}/{shorthand["NAME"]}-{file_name}.txt"
            print(output_path)

            tmpl = _tmpl.read()
            print(tmpl)



# for shorthand in shorthands: 
#     print("asdf")


