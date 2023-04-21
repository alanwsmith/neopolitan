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
        "blockquote": {"key": "BlockquoteSection"},
        "canvas": {"key": "CanvasSection"},
        "checklist": {"key": "ChecklistSection"},
        "code": {"key": "CodeSection"},
        "details": {"key": "DetailsSection"},
        "div": {"key": "DivSection"},
        "dlist": {"key": "DescriptionListSection"},
        "figure": {"key": "FigureSection"},
        "h1": {"key": "H1Section"},
        "h2": {"key": "H2Section"},
        "h3": {"key": "H3Section"},
        "h4": {"key": "H4Section"},
        "h5": {"key": "H5Section"},
        "h6": {"key": "H6Section"},
        "image": {"key": "ImageSection"},
        "list": {"key": "ListSection"},
        "menu": {"key": "MenuSection"},
        "nav": {"key": "NavSection"},
        "note": {"key": "NoteSection"},
        "notes": {"key": "NotesSection"},
        "object": {"key": "ObjectSection"},
        "olist": {"key": "OrderedListSection"},
        "p": {"key": "ParagraphsSection"},
        "picture": {"key": "PictureSection"},
        "pre": {"key": "PreSection"},
        "results": {"key": "ResultsSection"},
        "startcode": {"key": "CodeStartEndSection"},
        "subtitle": {"key": "SubtitleSection"},
        "table": {"key": "TableSection"},
        "textarea": {"key": "TextareaSection"},
        "title": {"key": "TitleSection"},
        "todo": {"key": "TodoSection"},
        "vimeo": {"key": "VimeoSection"},
        "warning": {"key": "WarningSection"},
        "youtube": {"key": "YouTubeSection"},
        "video": {"key": "VideoSection"},
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



def update_source_file():
    with open("../source_file/source_file.rs", "r") as _src:
        indata = _src.read()
        parts_a = indata.split("// AUTO GENERATED START: Sections //")
        parts_b = parts_a[1].split("// AUTO GENERATED END: Sections //")
    with open("../source_file/source_file2.rs", "w") as _out:
        _out.write(parts_a[0])
        _out.write("\n// AUTO GENERATED START: Sections //\n")
        for tag in base_files.keys():
            values = base_files[tag]
            enum = values["key"]
            _out.write(f"""              Section::{enum}""")
            _out.write("""{
                attributes,
                children,
            } => {
                let parts = joiner(children);
                output_string.push_str(
                    &base
                        .get_template("components/""")
            _out.write(tag)
            _out.write(""".j2")
                        .unwrap()
                        .render(context!(attributes, parts))
                        .unwrap()
                        .as_str(),
                );
            }
""")
        _out.write("\n// AUTO GENERATED END: Sections //\n")
        _out.write(parts_b[1])


def update_section_file():
    with open("../section/section.rs", "r") as _src:
        indata = _src.read()
        output_sections = []
        parts_a = indata.split("// AUTO GENERATED START: calls //")
        parts_b = parts_a[1].split("// AUTO GENERATED END: calls //")
        parts_c = parts_b[1].split("// AUTO GENERATED START: enum //")
        parts_d = parts_c[1].split("// AUTO GENERATED END: enum //")
        parts_e = parts_d[1].split("// AUTO GENERATED START: tags //")
        parts_f = parts_e[1].split("// AUTO GENERATED END: tags //")
        use_statements = []
        enums = []
        tags = []
        for tag in base_files.keys():
            print(tag)
            values = base_files[tag]
            enum = values["key"]
            use_statements.append(f"use crate::section::{tag}::*;")
            enums.append(enum)
            enums.append(""" {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },""")
            tags.append(
        f"""tuple((
            tag("-> {tag}"),
            not_line_ending,
            line_ending,
            alt((take_until("\\n\\n-> "), rest)),
        ))
        .map(|t| {tag}(t.3).unwrap().1),"""
                    )
        output_sections.append(
            parts_a[0]
        )
        output_sections.append(
            "// AUTO GENERATED START: calls //"
        )
        output_sections.append(
                "\n".join(use_statements)
        )
        output_sections.append(
            "// AUTO GENERATED END: calls //"
        )
        output_sections.append(
            parts_c[0]
        )
        output_sections.append(
            "// AUTO GENERATED START: enum //"
        )
        output_sections.append(
            "\n".join(enums)
        )
        output_sections.append(
            "// AUTO GENERATED END: enum //"
        )
        output_sections.append(
            parts_e[0]
        )
        output_sections.append(
            "// AUTO GENERATED START: tags //"
        )
        output_sections.append(
            "\n".join(tags)
        )
        output_sections.append(
            "// AUTO GENERATED END: tags //"
        )
        output_sections.append(
            parts_f[1]
        )
    with open("../section/section.rs", "w") as _out:
        _out.write("\n".join(output_sections))




if __name__ == "__main__":
    generate_section_files()
    update_section_mod_file()
    update_source_file()
    update_section_file()



from datetime import datetime 
timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
print(timestamp)
print("done")
