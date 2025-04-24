#!/usr/bin/env python3

import sqlite3
import sys
import os

from string import Template

script_dir = sys.path[0]
db_path = os.path.join(script_dir, "..", "_data.sqlite")
con = sqlite3.connect(db_path)
cur = con.cursor()

def make_basic_section_files():
    with open(os.path.join(script_dir, "_basic_section_template.rs")) as _in:
        skeleton = _in.read()
        template = Template(skeleton)
        sql = """
        SELECT tag, enum, status,type, rust_format 
        FROM sections 
        WHERE rust_format=? 
        """
        for row in cur.execute(sql, ("basic",)):
            data = {
                "TAG": row[0],
                "ENUM": row[1],
            }
            output_path = os.path.join(script_dir, f"{row[0]}.rs")
            output = template.substitute(data)
            with open(output_path, "w") as _out:
                _out.write(output)

    # output_path = os.path.join(script_dir, "content.neo")
    # with open(output_path, "w") as _out:


            # with open("templates/section_file.rs") as _in:
            #     skeleton = _in.read()
            #     template = Template(skeleton)
            #     output = template.substitute(data)
            #     print(output)
            #     with open(output_path, "w") as _out:
            #         _out.write(output)




        # sql = 'SELECT tag FROM sections WHERE type=? AND status=?'
        # for row in cur.execute(sql, ("content", "done")):
        #     lines.append(f"- <<{row[0]}|link|/sections/{row[0]}.html>>")
        #     lines.append("")
        # lines.append("-> h3")
        # lines.append("")
        # lines.append("In Progress")
        # lines.append("")
        # sql = 'SELECT tag FROM sections WHERE type=? AND status=?'
        # for row in cur.execute(sql, ("content", "todo")):
        #     lines.append(f"- <<{row[0]}|link|/sections/{row[0]}.html>>")
        #     lines.append("")
        # _out.write("\n".join(lines))

    con.close()

if __name__ == "__main__":
    make_basic_section_files()
    from datetime import datetime
    print(datetime.now())

