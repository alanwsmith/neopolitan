#!/usr/bin/env python3

import os.path

from string import Template

items_alfa = [
    ("aside", "AsideSection"),
    ("blockquote", "BlockquoteSection"),
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
]

def update_source_file():
    with open("../source_file/source_file.rs", "r") as _src:
        indata = _src.read()
        parts_a = indata.split("// AUTO GENERATED START: Sections //")
        parts_b = parts_a[1].split("// AUTO GENERATED END: Sections //")
    with open("../source_file/source_file.rs", "w") as _out:
        _out.write(parts_a[0])
        _out.write("\n\n// AUTO GENERATED START: Sections //\n\n")
        for item in items_alfa:
            _out.write(f"""              Section::{item[1]}""")
            _out.write("""{
                attributes,
                children,
            } => {
                let parts = joiner(children);
                output_string.push_str(
                    &base
                        .get_template("components/""")
            _out.write(item[0])
            _out.write(""".j2")
                        .unwrap()
                        .render(context!(attributes, parts))
                        .unwrap()
                        .as_str(),
                );
            }
""")
        _out.write("\n\n// AUTO GENERATED END: Sections //\n\n")
        _out.write(parts_b[1])

def update_mod_file():
    with open("../section/mod.rs", "w") as _out:
        for item in section_extras:
            _out.write(f"pub mod {item[0]};\n")
        for item in items_alfa:
            _out.write(f"pub mod {item[0]};\n")

def write_title_style_files():
    for item in title_style_sections: 
        with open("templates/title_style.rs") as _in:
            skeleton = _in.read()
            data = { "NAME1": item[0], "NAME2": item[1] }
            template = Template(skeleton)
            output = template.substitute(data)
            output_path = f"../section/{item[0]}.rs"
            if not os.path.isfile(output_path):
                print(f"Making: {output_path}")
                with open(output_path, "w") as _out:
                    _out.write(output)
            else: 
                print(f"Already exists: {output_path}")


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


        output_sections.append(
            parts_a[0]
        )
        output_sections.append(
            "// AUTO GENERATED START: calls //"
        )
        output_sections.append(
            "HERHERHER"
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
            "HEREHREHRE"
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
            "HEREHREHRE"
        )
        output_sections.append(
            "// AUTO GENERATED END: tags //"
        )
        output_sections.append(
            parts_f[1]
        )


        print(parts_b)
        # parts_c = 

    with open("../section/section_auto.rs", "w") as _out:
        _out.write("\n".join(output_sections))
    #     _out.write(parts_a[0])
    #     _out.write("\n\n// AUTO GENERATED START: Sections //\n\n")
    #     for item in items_alfa:
    #         _out.write(f"""              Section::{item[1]}""")
    #         _out.write("""{
    #             attributes,
    #             children,
    #         } => {
    #             let parts = joiner(children);
    #             output_string.push_str(
    #                 &base
    #                     .get_template("components/""")
    #         _out.write(item[0])
    #         _out.write(""".j2")
    #                     .unwrap()
    #                     .render(context!(attributes, parts))
    #                     .unwrap()
    #                     .as_str(),
    #             );
    #         }
# """)
    #     _out.write("\n\n// AUTO GENERATED END: Sections //\n\n")
    #     _out.write(parts_b[1])


update_source_file()
update_section_file()
update_mod_file()
write_title_style_files()
print("done")
