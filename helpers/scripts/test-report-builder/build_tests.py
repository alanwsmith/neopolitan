#!/usr/bin/env python3

import os

from pathlib import Path

file_list = []
for root, dirs, files in os.walk("../../.."):
    for file in files:
        file_path = os.path.join(root, file)
        file_list.append(file_path)

cases = {
            "block": {},
            "span": {}
        }

file_list.sort()

for path in file_list:
    ext = "".join(Path(path).suffixes)
    if ext == ".neotest":
        path_parts = path.split("/")
        category = path_parts[4]
        kind = path_parts[5]
        del path_parts[:6]
        key = "-".join(path_parts)
        if kind not in cases[category]:
            cases[category][kind] = []


print(cases)

