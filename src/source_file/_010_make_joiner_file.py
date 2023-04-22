#!/usr/bin/env python3

import sqlite3
import sys
import os

from string import Template

script_dir = sys.path[0]
db_path = os.path.join(script_dir, "..", "_data.sqlite")
output_path = os.path.join(script_dir, "joiner.rs")

def make_file():
    con = sqlite3.connect(db_path)
    cur = con.cursor()

    lines = []

    sql = 'SELECT enum FROM inline_tags WHERE format=?'
    for row in cur.execute(sql, ("basic", )):
        lines.append(f"""Snippet::{row[0]}""")
        lines.append("""    { string } => {""")
        lines.append("""   assembler.push(string.as_ref().unwrap().to_string()); }""")
    data = { "ENUMS": "\n".join(lines) }

    with open(os.path.join(script_dir, "_joiner_template.rs")) as _in:
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

