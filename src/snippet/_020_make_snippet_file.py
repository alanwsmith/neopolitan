#!/usr/bin/env python3

import sqlite3
import sys
import os

from string import Template

script_dir = sys.path[0]
db_path = os.path.join(script_dir, "..", "_data.sqlite")
output_path = os.path.join(script_dir, "snippet.rs")

def make_file():
    con = sqlite3.connect(db_path)
    cur = con.cursor()

    usage_lines = []

    lines = [
[],
[],
[],
[],
[],
    ]

    sql = 'SELECT tag FROM inline_tags WHERE format=? ORDER BY tag DESC'
    counter = 0
    for row in cur.execute(sql, ("basic", )):
        usage_lines.append(f"use crate::snippet::snippets::{row[0]}::{row[0]};")
        lines[counter % 5].append(f"""
        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("{row[0]}"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| {row[0]}(x.2, x.6)),
""")

        counter += 1

    data = { 
        "USAGE": "\n".join(usage_lines),
        "LINES0": "\n".join(lines[0]),
        "LINES1": "\n".join(lines[1]),
        "LINES2": "\n".join(lines[2]),
        "LINES3": "\n".join(lines[3]),
        "LINES4": "\n".join(lines[4]),
        }

    with open(os.path.join(script_dir, "_snippet_template.rs")) as _in:
        skeleton = _in.read()
        template = Template(skeleton)
        output = template.substitute(data)

        print(output)

        with open(output_path, "w") as _out:
            _out.write(output)

    con.close()

if __name__ == "__main__":
    make_file()
    from datetime import datetime
    print(datetime.now())

