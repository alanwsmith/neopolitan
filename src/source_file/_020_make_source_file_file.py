#!/usr/bin/env python3

import sqlite3
import sys
import os

from string import Template

script_dir = sys.path[0]
db_path = os.path.join(script_dir, "..", "_data.sqlite")
template_path = os.path.join(script_dir, "_template_source_file.rs")
output_path = os.path.join(script_dir, "source_file.rs")

def make_file():
    con = sqlite3.connect(db_path)
    cur = con.cursor()

    lines = []

    sql = """SELECT tag, enum, type, template, attributes
    FROM sections WHERE type=? """
    for row in cur.execute(sql, ("content", )):
        lines.append(f"Section::{row[1]}")
        lines.append("{ attributes, children,} => {")
        lines.append("let parts = joiner(children);")
        if row[4] == "with_class":
            lines.append(f"""let attributes_string = attributes_{row[4]}(attributes, "{row[0]}");""")
        else:
            lines.append(f"""let attributes_string = attributes_{row[4]}(attributes);""")
        lines.append("output_string.push_str(&base")
        lines.append(f""".get_template("components/{row[0]}.j2")""")
        lines.append(".unwrap().render(context!(attributes_string, parts))")
        lines.append(".unwrap().as_str());")
        lines.append("}")

    sql = """SELECT tag, enum, type, template, attributes
    FROM sections WHERE tag=?"""
    for row in cur.execute(sql, ("startneoexample", )):
        lines.append(f"Section::{row[1]}")
        lines.append("{ attributes, html, raw} => {")
        #lines.append("let parts = joiner(children);")
        lines.append(f"""let attributes_string = attributes_{row[4]}(attributes);""")
        lines.append("output_string.push_str(&base")
        lines.append(f""".get_template("components/{row[0]}.j2")""")
        lines.append(".unwrap().render(context!(attributes_string, html, raw))")
        lines.append(".unwrap().as_str());")
        lines.append("}")


    data = {
        "SECTIONDATA": "\n".join(lines)
    }

    with open(template_path) as _in:
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




