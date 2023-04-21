import sqlite3
import sys
import os

script_dir = sys.path[0]
db_path = os.path.join(script_dir, "build_data.sqlite")
con = sqlite3.connect(db_path)
cur = con.cursor()

output_path = os.path.join(
        script_dir, 
        "..", 
        "..", 
        "site", 
        "content", 
        "sections.neo"
    )
with open(output_path, "w") as _out:
    lines = []
    lines.append("-> title")
    lines.append("")
    lines.append("Sections")
    lines.append("")
    sql = 'SELECT tag FROM sections'
    for row in cur.execute(sql):
        lines.append(f"- <<{row[0]}|link|/sections/{row[0]}.html>>")
        lines.append("")



    _out.write("\n".join(lines))
con.close()



from datetime import datetime
print(datetime.now())


