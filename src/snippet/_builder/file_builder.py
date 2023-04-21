#!/usr/bin/env python3

import sqlite3
import sys
import os

from datetime import datetime
from pathlib import Path
from string import Template

script_dir = sys.path[0]
db_path = os.path.join(script_dir, "..", "..", "_builder", "_data.sqlite")
snippet_dir = os.path.join(script_dir, "..")
templates_dir = os.path.join(script_dir, "templates", "files")

def build_files():
    con = sqlite3.connect(db_path)
    cur = con.cursor()
    with open(os.path.join(templates_dir, "default.rs")) as _in:
        skeleton = _in.read()
        template = Template(skeleton)
    sql = 'SELECT tag FROM inline_tags'
    for row in cur.execute(sql):
        output_path = os.path.join(snippet_dir, f"{row[0]}.rs")
        data = {
            "TAG": row[0]
        }
        if not Path(output_path).is_file():
            output = template.substitute(data)
            with open(output_path, "w") as _out:
                _out.write(output)
    con.close()

if __name__ == "__main__":
    build_files()
    print(f"Done: {datetime.now()}")

