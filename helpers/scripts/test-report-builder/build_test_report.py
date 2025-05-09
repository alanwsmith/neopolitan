#!/usr/bin/env python3

import os

from html import escape
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
        key = ".".join(path_parts)
        if kind not in cases[category]:
            cases[category][kind] = []
        with open(path) as _case:
            case = _case.read()
            case_parts = case.split("######")
            if case_parts[2] == "ok":
                pass

                #case_id = escape(key.replace("-", "-").strip())
                #description = escape(case_parts[3].replace("-", "-").strip())
                #given = escape(case_parts[0].replace("-", "-").strip())
                #expected = escape(case_parts[4].replace("-", "-").strip())
                #remainder = escape(case_parts[5].replace("-", "-").strip())
                #payload = f"""
#<div class="padding-xsmall border-color-alt-1-80 margin-bottom-medium border-radius-medium">
    #<div class="padding-xsmall font-size-xsmall border-bottom-color-alt-1-80 color-primary-50">{description}</div>
    #<pre class="font-size-xsmall border-bottom-color-alt-1-80 padding-bottom-small">{given}</pre>
    #<pre class="font-size-xsmall margin-top-xsmall color-primary-70">{escape(case_parts[5].strip())}</pre>
    #<details>
        #<summary class="font-size-xsmall margin-top-xsmall padding-bottom-small color-primary-70">Expected</summary>
        #<pre class="font-size-xsmall color-primary-70">{escape(case_parts[4].strip())}</pre>
    #</details>
    #<div class="font-size-xsmall border-top-color-alt-3-60 color-primary-30">{case_id}</div>
#</div>
#"""
                #cases[category][kind].append(payload)
                ##print(case)
                ##print(payload)




# with open("../../../docs-content/_test-report.neo", "w") as _out:
#     _out.write("-- title\n\nTest Report\n\n-- html/\n\n")
#     _out.write('<div class="cases">')
#     for cat in cases:
#         _out.write('<div class="category-cases">')
#         for kind in cases[cat]:
#             _out.write(f"""<details clsss="case-kind"><summary class="margin-xxsmall">{cat} - {kind}</summary>""")
#             for payload in cases[cat][kind]:
#                 _out.write(payload)
#             _out.write('</details>')
#         _out.write('</div>')
#     _out.write('</div>')
#     _out.write("-- title\n\nTest Report\n\n-- /html\n\n")




        #cases[category][kind].append(key)



print(cases)

