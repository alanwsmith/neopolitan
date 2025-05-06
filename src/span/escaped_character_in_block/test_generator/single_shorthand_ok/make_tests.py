#!/usr/bin/env python3

import glob
import os

from string import Template

template_dir = "templates"
output_dir = "../../tests/ok"

file_names = [
    os.path.basename(file) for file in glob.glob(f"{template_dir}/*")
    if os.path.isfile(file)
]


with open("../../../../helpers/data/characters-to-escape-in-blocks.txt", "r") as _char_file:
    lines = _char_file.read().splitlines()
    for line in lines:
        parts = line.split(" ")
        if len(parts) >= 2:
            payload = {
                    "NAME": parts[1],
                    "TOKEN": parts[0]
                    }
            for file_name in file_names:
                with open(f"{template_dir}/{file_name}", "r") as _tmpl:
                    output_path = f"{output_dir}/generated-{payload["NAME"]}-{file_name}"
                    print(output_path)
                    template = Template(_tmpl.read())
                    output = template.substitute(payload)
                    with open(output_path, "w") as _out:
                        _out.write(output)

