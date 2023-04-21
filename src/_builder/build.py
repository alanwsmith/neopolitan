#!/usr/bin/env python3

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

def update_source_file():
    with open("../source_file/source_file.rs", "r") as _src:
        indata = _src.read()
        parts_a = indata.split("// AUTO GENERATED START: Sections //")
        parts_b = parts_a[1].split("// AUTO GENERATED END: Sections //")
        with open("../source_file/test.rs", "w") as _out:
            _out.write(parts_a[0])

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




                print(item)
            _out.write(parts_b[1])


        # print(indata)


update_source_file()
print("done")

