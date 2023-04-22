#!/usr/bin/env python3

import sqlite3
import sys
import os

from string import Template

script_dir = sys.path[0]
db_path = os.path.join(script_dir, "..", "..", "src", "_data.sqlite")

def make_files():
    con = sqlite3.connect(db_path)
    cur = con.cursor()
    with open(os.path.join(script_dir, "_basic_template.rs")) as _in:
        skeleton = _in.read()
    sql = 'SELECT tag, enum FROM inline_tags WHERE format=?'

    for row in cur.execute(sql, ("basic", )):
        output_path = os.path.join(script_dir, f"{row[0]}.rs")
        data = { "TAG": row[0], "ENUM": row[1] }
        print(data)
        template = Template(skeleton)
        output = template.substitute(data)
        print(output)
        with open(output_path, "w") as _out:
            _out.write(output)
    con.close()

if __name__ == "__main__":
    make_files()
    from datetime import datetime
    print(datetime.now())


