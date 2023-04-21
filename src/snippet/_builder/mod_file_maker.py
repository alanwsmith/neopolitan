#!/usr/bin/env python3

import sqlite3
import sys
import os

from datetime import datetime

script_dir = sys.path[0]
db_path = os.path.join(script_dir, "..", "..", "_builder", "_data.sqlite")
mod_file_path = os.path.join(script_dir, "..", "mod.rs")

def make_mod_file():
    con = sqlite3.connect(db_path)
    cur = con.cursor()
    with open(mod_file_path, "w") as _out:
        #
        # Hard coded
        #
        _out.write("pub mod get_attributes;\n")
        _out.write("pub mod snippet;\n")
        #
        #
        sql = 'SELECT tag FROM inline_tags'
        for row in cur.execute(sql):
            print(row[0])
            _out.write(f"pub mod {row[0]};\n")
    con.close()

if __name__ == "__main__":
    make_mod_file()
    print(f"Done: {datetime.now()}")

