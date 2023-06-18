#!/usr/bin/env python3

import sqlite3
import sys
import os

script_dir = sys.path[0]
db_path = os.path.join(script_dir, "..", "..", "..", "src", "_data.sqlite")
con = sqlite3.connect(db_path)
cur = con.cursor()

output_path = os.path.join(script_dir, "content.neo")
with open(output_path, "w") as _out:
    lines = []
    lines.append("-> title")
    lines.append("")
    lines.append("Neopolitan")
    lines.append("")
    lines.append("-> h2")
    lines.append("")
    lines.append("Content Sections")
    lines.append("")
    sql = 'SELECT tag FROM sections WHERE type=? AND status=?'
    for row in cur.execute(sql, ("content", "done")):
        lines.append(f"- <<{row[0]}|link|/sections/{row[0]}.html>>")
        lines.append("")
    lines.append("-> h3")
    lines.append("")
    lines.append("In Progress")
    lines.append("")
    sql = 'SELECT tag FROM sections WHERE type=? AND status=?'
    for row in cur.execute(sql, ("content", "todo")):
        lines.append(f"- <<{row[0]}|link|/sections/{row[0]}.html>>")
        lines.append("")
    _out.write("\n".join(lines))


output_path = os.path.join(script_dir, "metadata.neo")
with open(output_path, "w") as _out:
    lines = []
    lines.append("-> title")
    lines.append("")
    lines.append("Neopolitan")
    lines.append("")
    lines.append("-> h2")
    lines.append("")
    lines.append("Metadata Sections")
    lines.append("")
    sql = 'SELECT tag FROM sections WHERE type=? AND status=?'
    for row in cur.execute(sql, ("meta", "done")):
        lines.append(f"- <<{row[0]}|link|/sections/{row[0]}.html>>")
        lines.append("")
    lines.append("-> h3")
    lines.append("")
    lines.append("In Progress")
    lines.append("")
    sql = 'SELECT tag FROM sections WHERE type=? AND status=?'
    for row in cur.execute(sql, ("meta", "todo")):
        lines.append(f"- <<{row[0]}|link|/sections/{row[0]}.html>>")
        lines.append("")
    _out.write("\n".join(lines))


output_path = os.path.join(script_dir, "functional.neo")
with open(output_path, "w") as _out:
    lines = []
    lines.append("-> title")
    lines.append("")
    lines.append("Neopolitan")
    lines.append("")
    lines.append("-> h2")
    lines.append("")
    lines.append("Functional Sections")
    lines.append("")
    sql = 'SELECT tag FROM sections WHERE type=? AND status=?'
    for row in cur.execute(sql, ("meta", "done")):
        lines.append(f"- <<{row[0]}|link|/sections/{row[0]}.html>>")
        lines.append("")
    lines.append("-> h3")
    lines.append("")
    lines.append("In Progress")
    lines.append("")
    sql = 'SELECT tag FROM sections WHERE type=? AND status=?'
    for row in cur.execute(sql, ("meta", "todo")):
        lines.append(f"- <<{row[0]}|link|/sections/{row[0]}.html>>")
        lines.append("")
    _out.write("\n".join(lines))

con.close()

from datetime import datetime
print(datetime.now())

