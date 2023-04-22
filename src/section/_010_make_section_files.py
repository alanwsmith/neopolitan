#!/usr/bin/env python3

import sqlite3
import sys
import os

from string import Template

script_dir = sys.path[0]
db_path = os.path.join(script_dir, "..", "_data.sqlite")
con = sqlite3.connect(db_path)
cur = con.cursor()

def make_section_files():
    print("Making section files")
    sql = """
    SELECT tag, enum, type, template 
    FROM sections 
    WHERE template IS NOT NULL
    """
    for row in cur.execute(sql):
        data = {
            "TAG": row[0],
            "ENUM": row[1]
        }
        with open(os.path.join(script_dir, f"_template_{row[3]}.rs")) as _in:
            skeleton = _in.read()
            template = Template(skeleton)
            output_path = os.path.join(script_dir, f"{row[0]}.rs")
            output = template.substitute(data)
            with open(output_path, "w") as _out:
                _out.write(output)
            print(output_path)

if __name__ == "__main__":
    make_section_files()
    from datetime import datetime
    print(datetime.now())

