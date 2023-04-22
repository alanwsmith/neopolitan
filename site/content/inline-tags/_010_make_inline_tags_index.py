#!/usr/bin/env python3


import sqlite3
import sys
import os

script_dir = sys.path[0]
db_path = os.path.join(script_dir, "..", "..", "..", "src", "_data.sqlite")
con = sqlite3.connect(db_path)
cur = con.cursor()

output_path = os.path.join(
        script_dir, 
        "index.neo"
    )
with open(output_path, "w") as _out:
    lines = []
    lines.append("-> title")
    lines.append("")
    lines.append("Inline Tags")
    lines.append("")
    lines.append("")
    sql = 'SELECT tag FROM inline_tags WHERE format=?'
    for row in cur.execute(sql, ("basic",)):
        lines.append(f"- <<{row[0]}|link|/inline-tags/{row[0]}.html>>")
        lines.append("")
    _out.write("\n".join(lines))
con.close()

from datetime import datetime
print(datetime.now())

