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
    sql = 'SELECT tag, description FROM inline_tags WHERE format=? OR format=?'
    for row in cur.execute(sql, ("basic", "link")):
        lines.append(f"- <<{row[0]}|link|/inline-tags/{row[0]}.html>> - {row[1]}")
        lines.append("")
    lines.append("")
    lines.append("-> h3")
    lines.append("")
    lines.append("In Progress")
    lines.append("")

    sql = 'SELECT tag, description FROM inline_tags WHERE format=?'
    for row in cur.execute(sql, ("tbd",)):
        lines.append(f"- {row[0]} - {row[1]}")
        lines.append("")


    _out.write("\n".join(lines))
con.close()

from datetime import datetime
print(datetime.now())

