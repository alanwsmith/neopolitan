import sqlite3
import sys
import os

script_dir = sys.path[0]
db_path = os.path.join(script_dir, "_data.sqlite")
con = sqlite3.connect(db_path)
cur = con.cursor()

inline_tags = [
("audio","AudioTag"),
("b","BringAttentionTag"),
("button","ButtonTag"),
("cite","CiteTag"),
("code","CodeTag"),
("data","DataTag"),
("del","DeleteTag"),
("dfn","DefinitionTag"),
("em","EmphasisTag"),
("i","IdiomaticTextTag"),
("img","ImageTag"),
("input","InputTag"),
("ins","InsertTag  "),
("kbd","KeyboardInput"),
("label","LabelTag"),
("legend","LegendTag"),
("link","LinkTag"),
("meter","MeterTag"),
("object","ObjectTag"),
("progress","ProgressTag"),
("q","QuotationTag"),
("s","StrikethroughTag "),
("samp","SampleOutputTag"),
("small","SmallTextTag"),
("span","SpanTag"),
("strong","StrongTag"),
("sub","SubscriptTag"),
("sup","SuperscriptTag"),
("time","TimeTag"),
("u","UnarticulatedAnnotationTag"),
("var","VariableTag"),
("wbr","LineBreakOpportunityTag"),
]

def insert_inline_tags():
    sql = 'INSERT INTO inline_tags (tag, enum) VALUES (?, ?)'
    for tag in inline_tags:
        cur.execute(sql, (tag[0], tag[1]))
        print(tag[0])
    con.commit()
    con.close()

if __name__ == "__main__":
    # NOTE: This is a one off script to load
    # data into the sqlite database. The
    # table is setup with a unique index, but
    # still turning this off here
    # insert_inline_tags()
    print("done")


