#!/usr/bin/env python3

import sqlite3
import sys
import os

script_dir = sys.path[0]
db_path = os.path.join(script_dir, "..", "_data.sqlite")
output_path = os.path.join(script_dir, "snippet_enum.rs")

def make_file():
    con = sqlite3.connect(db_path)
    cur = con.cursor()

    lines = [
"use serde::Serialize;"
"",
"#[derive(Debug, PartialEq, Serialize)]",
'#[serde(tag = "type")]',
"pub enum Snippet {",
"   Plain {text: Option<String>},"
]

    sql = 'SELECT enum FROM inline_tags WHERE format=?'
    for row in cur.execute(sql, ("basic", )):
        lines.append(row[0])
        lines.append("{ string: Option<String> },")

    lines.append("}")
    with open(output_path, "w") as _out:
        _out.write("\n".join(lines))

    con.close()

if __name__ == "__main__":
    make_file()
    from datetime import datetime
    print(datetime.now())

