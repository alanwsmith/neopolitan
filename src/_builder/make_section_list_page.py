import sqlite3
import sys
import os


db_path = os.path.join(sys.path[0], "build_data.sqlite")
con = sqlite3.connect(db_path)
cur = con.cursor()



con.close()


