#!/usr/bin/env python3

import os.path

from string import Template



base_files = {
        "aside": {"key": "AsideSection", "example": "\nShut the hatch"},
        "attributes": {"key": "AttributesSection"},
        "blockquote": {"key": "BlockquoteSection"},
        "blurb": {"key": "BlurbSection"},
        "canvas": {"key": "CanvasSection"},
        "categories": {"key": "CategoriesSection"},
        "checklist": {"key": "ChecklistSection"},
        "code": {"key": "CodeSection"},
        "comment": {"key": "CommentSection"},
        "css": {"key": "CSSSection"},
        "details": {"key": "DetailsSection"},
        "div": {"key": "DivSection"},
        "dlist": {"key": "DescriptionListSection"},
        "ext": {"key": "ExternalSection"},
        "figure": {"key": "FigureSection"},
        "footnote": {"key": "FootnoteSection"},
        "h1": {"key": "H1Section"},
        "h2": {"key": "H2Section"},
        "h3": {"key": "H3Section"},
        "h4": {"key": "H4Section"},
        "h5": {"key": "H5Section"},
        "h6": {"key": "H6Section"},
        "head": {"key": "HeadSection"},
        "hr": {"key": "HRSection"},
        "html": {"key": "HTMLSection"},
        "image": {"key": "ImageSection"},
        "include": {"key": "IncludeSection"},
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
        "reference": {"key": "ReferenceSection"},
        "results": {"key": "ResultsSection"},
        "script": {"key": "ScriptSection"},
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
        "widget": {"key": "WidgetSection"},

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
    with open("../source_file/source_file.rs", "w") as _out:
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


def make_templates():
    for tag in base_files.keys():
        output_path = f"../../site/templates/components/{tag}.j2"
        if not os.path.isfile(output_path):
            with open(output_path, "w") as _out:
                _out.write(f"TEMPLATE_TODO: {tag}")


def make_example():
    with open("../../site/content/sections.neo", "w") as _out:
        lines = ["-> title"]
        lines.append("")
        lines.append("Section Examples")
        lines.append("")
        for tag in base_files.keys():
            lines.append("-> h3")
            lines.append("")
            lines.append(f"{tag}")
            lines.append("")
            lines.append(f"-> {tag}")
            lines.append("")
            lines.append("x")
            lines.append("")

        _out.write("\n".join(lines))





if __name__ == "__main__":
    print("Probably don't use this. it was for initial generation")
    
    # These will overrides somethings but not
    # everything. use with care if you need to 
    # redo batches of things
    #generate_section_files()
    #update_source_file()
    #update_section_file()
    #make_templates()
    #make_example()

    # Remember that this needs to write everything
    # note just new ones if you add them
    update_section_mod_file()

from datetime import datetime 
timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
print(timestamp)
print("done")
