#!/usr/bin/env python3

import os.path

from string import Template

items_alfa = [
    ("aside", "AsideSection"),
    ("blockquote", "BlockquoteSection"),
    ("canvas", "CanvasSection"),
    ("h1", "H1Section"),
    ("h2", "H2Section"),
    ("h3", "H3Section"),
    ("h4", "H4Section"),
    ("h5", "H5Section"),
    ("h6", "H6Section"),
    ("note", "NoteSection"),
    ("subtitle", "SubtitleSection"),
    ("title", "TitleSection")
]

section_extras = [
    ("section", ),
    ("section_attributes", ),
]

# Title style outputs that have a top element
# followed by paragraphs

title_style_sections = [
    ("h1", "H1Section"),
    ("h2", "H2Section"),
    ("h3", "H3Section"),
    ("h4", "H4Section"),
    ("h5", "H5Section"),
    ("h6", "H6Section"),
    ("title", "TitleSection")
]

# full section list
full_section_list = [
    ("aside", "AsideSection"),
    ("blockquote", "BlockquoteSection"),
    ("canvas", "CanvasSection"),
    ("h1", "H1Section"),
    ("h2", "H2Section"),
    ("h3", "H3Section"),
    ("h4", "H4Section"),
    ("h5", "H5Section"),
    ("h6", "H6Section"),
    ("note", "NoteSection"),
    ("subtitle", "SubtitleSection"),
    ("title", "TitleSection")
]


base_files = {
        "aside": {"key": "AsideSection"},
        "blockquote": {"key": "BlockquoteSeciton"},
        "canvas": {"key": "CanvasSeciton"},
        "checklist": {"key": "ChecklistSeciton"},
        "code": {"key": "CodeSeciton"},
        "details": {"key": "DetailsSeciton"},
        "div": {"key": "DivSeciton"},
        "dlist": {"key": "DescriptionListSeciton"},
        "figure": {"key": "FigureSeciton"},
        "h1": {"key": "H1Seciton"},
        "h2": {"key": "H2Seciton"},
        "h3": {"key": "H3Seciton"},
        "h4": {"key": "H4Seciton"},
        "h5": {"key": "H5Seciton"},
        "h6": {"key": "H6Seciton"},
        "image": {"key": "ImageSeciton"},
        "list": {"key": "ListSeciton"},
        "menu": {"key": "MenuSeciton"},
        "nav": {"key": "NavSeciton"},
        "note": {"key": "NoteSeciton"},
        "notes": {"key": "NotesSeciton"},
        "object": {"key": "ObjectSeciton"},
        "olist": {"key": "OrderedListSeciton"},
        "p": {"key": "ParagraphsSeciton"},
        "picture": {"key": "PictureSeciton"},
        "pre": {"key": "PreSeciton"},
        "results": {"key": "ResultsSeciton"},
        "startcode": {"key": "CodeStartEndSeciton"},
        "subtitle": {"key": "SubtitleSeciton"},
        "table": {"key": "TableSeciton"},
        "textarea": {"key": "TextareaSeciton"},
        "title": {"key": "TitleSeciton"},
        "todo": {"key": "TodoSeciton"},
        "vimeo": {"key": "VimeoSeciton"},
        "warning": {"key": "WarningSeciton"},
        "youtube": {"key": "YouTubeSeciton"},
        "video": {"key": "VideoSeciton"},
        }

def generate_section_files():
    for tag in base_files.keys():
        values = base_files[tag]
        enum = values["key"]
        data = {
            "KEY": tag,
            "ENUM": enum,
        }
        output_path = f"../section/{tag}.rs"
        if not os.path.isfile(output_path):
            print(f"Making: {output_path}")
            with open("templates/section_file.rs") as _in:
                skeleton = _in.read()
                template = Template(skeleton)
                output = template.substitute(data)
                print(output)
                with open(output_path, "w") as _out:
                    _out.write(output)
        else:
            print(f"Already exists: {output_path}")



def update_section_mod_file():
    output_path = "../section/mod.rs"
    if not os.path.isfile(output_path):
        with open(output_path, "w") as _out:
            _out.write("pub mod section;\n");
            _out.write("pub mod section_attributes;\n");
            for tag in base_files.keys():
                _out.write(f"pub mod {tag};\n");





# def update_source_file():
#     with open("../source_file/source_file.rs", "r") as _src:
#         indata = _src.read()
#         parts_a = indata.split("// AUTO GENERATED START: Sections //")
#         parts_b = parts_a[1].split("// AUTO GENERATED END: Sections //")
#     with open("../source_file/source_file.rs", "w") as _out:
#         _out.write(parts_a[0])
#         _out.write("\n\n// AUTO GENERATED START: Sections //\n\n")
#         for item in items_alfa:
#             _out.write(f"""              Section::{item[1]}""")
#             _out.write("""{
#                 attributes,
#                 children,
#             } => {
#                 let parts = joiner(children);
#                 output_string.push_str(
#                     &base
#                         .get_template("components/""")
#             _out.write(item[0])
#             _out.write(""".j2")
#                         .unwrap()
#                         .render(context!(attributes, parts))
#                         .unwrap()
#                         .as_str(),
#                 );
#             }
# """)
#         _out.write("\n\n// AUTO GENERATED END: Sections //\n\n")
#         _out.write(parts_b[1])

# def update_mod_file():
#     with open("../section/mod.rs", "w") as _out:
#         for item in section_extras:
#             _out.write(f"pub mod {item[0]};\n")
#         for item in items_alfa:
#             _out.write(f"pub mod {item[0]};\n")

# def write_title_style_files():
#     for item in title_style_sections: 
#         with open("templates/title_style.rs") as _in:
#             skeleton = _in.read()
#             data = { "NAME1": item[0], "NAME2": item[1] }
#             template = Template(skeleton)
#             output = template.substitute(data)
#             output_path = f"../section/{item[0]}.rs"
#             if not os.path.isfile(output_path):
#                 print(f"Making: {output_path}")
#                 with open(output_path, "w") as _out:
#                     _out.write(output)
#             else: 
#                 print(f"Already exists: {output_path}")


# def update_section_file():
#     with open("../section/section.rs", "r") as _src:
#         indata = _src.read()
#         output_sections = []
#         parts_a = indata.split("// AUTO GENERATED START: calls //")
#         parts_b = parts_a[1].split("// AUTO GENERATED END: calls //")
#         parts_c = parts_b[1].split("// AUTO GENERATED START: enum //")
#         parts_d = parts_c[1].split("// AUTO GENERATED END: enum //")
#         parts_e = parts_d[1].split("// AUTO GENERATED START: tags //")
#         parts_f = parts_e[1].split("// AUTO GENERATED END: tags //")
#         use_statements = []
#         enums = []
#         tags = []
#         for item in full_section_list:
#             use_statements.append(f"use crate::section::{item[0]}::*;")
#             enums.append(item[1])
#             enums.append(""" {
#         attributes: Option<Vec<SectionAttribute>>,
#         children: Option<Vec<Block>>,
#     },""")
#             tags.append(
#         f"""tuple((
#             tag("-> {item[0]}"),
#             not_line_ending,
#             line_ending,
#             alt((take_until("\\n\\n-> "), rest)),
#         ))
#         .map(|t| {item[0]}(t.3).unwrap().1),"""
#                     )
#             print(item)
#         output_sections.append(
#             parts_a[0]
#         )
#         output_sections.append(
#             "// AUTO GENERATED START: calls //"
#         )
#         output_sections.append(
#                 "\n".join(use_statements)
#         )
#         output_sections.append(
#             "// AUTO GENERATED END: calls //"
#         )
#         output_sections.append(
#             parts_c[0]
#         )
#         output_sections.append(
#             "// AUTO GENERATED START: enum //"
#         )
#         output_sections.append(
#             "\n".join(enums)
#         )
#         output_sections.append(
#             "// AUTO GENERATED END: enum //"
#         )
#         output_sections.append(
#             parts_e[0]
#         )
#         output_sections.append(
#             "// AUTO GENERATED START: tags //"
#         )
#         output_sections.append(
#             "\n".join(tags)
#         )
#         output_sections.append(
#             "// AUTO GENERATED END: tags //"
#         )
#         output_sections.append(
#             parts_f[1]
#         )
#     with open("../section/section.rs", "w") as _out:
#         _out.write("\n".join(output_sections))





# update_source_file()
# update_section_file()
# update_mod_file()
# write_title_style_files()

if __name__ == "__main__":
    generate_section_files()
    update_section_mod_file()



from datetime import datetime 
timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
print(timestamp)
print("done")
