#!/usr/bin/env python3

import glob
import os
import sys

from string import Template
from pathlib import Path 

data_file = "../../../../helpers/data/characters-to-escape-in-blocks.txt"
output_root = "../../tests/generated-ok"
test_name = os.path.basename(sys.path[0])
output_dir = f"{output_root}/{test_name}"
Path(output_dir).mkdir(exist_ok=True)

file_list = [
    file for file in glob.glob(f"{output_dir}/*.neotest")
    if os.path.isfile(file)
]
for file in file_list:
    os.remove(file)


with open(data_file, "r") as _char_file:
    lines = _char_file.read().splitlines()
    for line in lines:
        parts = line.split(" ")
        if len(parts) >= 2:
            payload = {
                    "NAME": parts[1],
                    "TOKEN": parts[0]
                    }
            with open(f"template.neotesttmpl", "r") as _tmpl:
                output_path = f"{output_dir}/{payload["NAME"]}.neotest"
                print(output_path)
                template = Template(_tmpl.read())
                output = template.substitute(payload)
                with open(output_path, "w") as _out:
                    _out.write(output)

