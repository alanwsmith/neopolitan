import json
import re

class BasicPost ():

    def __init__(self):
        self._body = []
        self._links = []
        self._images = None


    def body(self):
        self._body = []
        sections = re.split(r'^---:\s+', self.text, flags=re.M)
        for section in sections:
            lines = section.split("\n")
            section_type = lines[0].split(" ")[0]
            if section_type == "CONTENT":
                self.process_content_v2(lines)
        body = "\n".join(self._body)
        return body


    def publish(self, payload):
        out = None
        t = payload['type']
        c = payload['content']
        if t == 'h1':
            out = f"<h1>{c}</h1>"
        if payload['type'] == 'h2':
            out = f"<h2>{c}</h2>"
        if payload['type'] == 'h3':
            out = f"<h3>{c}</h3>"
        if payload['type'] == 'h4':
            out = f"<h4>{c}</h4>"
        if payload['type'] == 'h5':
            out = f"<h5>{c}</h5>"
        if payload['type'] == 'h6':
            out = f"<h6>{c}</h6>"


        if out != None:
            self._body.append(out)



    def process_content_v2(self, lines):
        thing = {
            "type": None,
            "content": None
        }
        for line in lines[1:]:
            m_header = re.search(r'^(#+) (.*)', line)
            if m_header:
                thing['type'] = f"h{len(m_header.group(1))}"
                thing['content'] = m_header.group(2)
                self.publish(thing)







        

    def body_v1(self):
        self._body = []
        sections = re.split(r'^---:\s+', self.text, flags=re.M)
        for section in sections:
            lines = section.split("\n")
            section_type = lines[0].split(" ")[0]
            if section_type == "CONTENT":
                self._body.extend(self.process_content(lines))
            elif section_type == "TODO":
                self._body.extend(self.process_todo(lines))
            elif section_type == "HTML":
                self._body.extend(self.process_html(lines))
            elif section_type == "WIDGET":
                self._body.extend(self.process_widget(lines))
            elif section_type == "NOTES":
                self._body.extend(self.process_notes(lines))
            elif section_type == "DIV":
                self._body.extend(self.process_div(lines))
        return "\n".join(self._body)


    def prep_line(self, line):
        m = re.search(r'<<(.+?)\|(.*?)>>', line)
        if m:
            parts = m.group(2).split("|")

            if m.group(1) == 'footnote':
                line = line.replace(m.group(), f'<sup>{m.group(2)}</sup>')
            if m.group(1) == 'image':
                img = self._images[int(m.group(2))]
                src = img['src']
                alt = self.handle_escapes(img['alt'])
                tag = f'<img class="post-image" src="{src}" alt="{alt}" />'
                line = line.replace(m.group(), tag)
            if m.group(1) == 'code':
                tag = ['<code']
                if len(parts) > 1: 
                    tag.append(' ')
                    tag.append(f'class="language-{parts[1]}"')
                tag.append(f'>{parts[0]}</code>')
                line = line.replace(m.group(), "".join(tag))
                

        return line


    def handle_links(self, line):
        # TODO: move this into process_line
        m = re.search(r'<<link\|(\d+)\|(.*?)>>', line)
        if m:
            href = self._links[int(m.group(1))]['href']
            aria = self._links[int(m.group(1))]['aria-label']
            text = m.group(2)
            link = f'<a href="{href}" aria-label="{aria}">{text}</a>'
            line = line.replace(f"{m.group()}", link)
        return line

    def handle_escapes(self, line):
        # TODO: move this into process_line
        line = line.replace("&", '&amp;')
        line = line.replace('"', '&quot;')
        line = line.replace("'", '&#39;')
        line = re.sub(r'(?<!<)<(?!<)', '&lt;', line)
        line = re.sub(r'(?<!>)>(?!>)', '&gt;', line)
        return line

    def load_file(self, text):
        self.text = text
        sections = re.split(r'^---:\s+', self.text, flags=re.M)
        for section in sections:
            lines = section.split("\n")
            section_type = lines[0].split(" ")[0]
            if section_type == "LINKS":
                payload = "\n".join(lines[1:])
                self._links = json.loads(payload)
            if section_type == "IMAGES":
                payload = "\n".join(lines[1:])
                self._images = json.loads(payload)

    def process_content_v1(self, lines):
        content = []
        tokens = lines[0].split(":")

        # TODO: Remove this since header stuff 
        # is on secondary lines for now
        if len(tokens) > 1:
            parts = tokens[1].split(" ")
            classes = " ".join(parts[1:])
            opening_div = f'<div class="{classes}">'

        line_type = None
        offload = False
        assembler = []
        area_type = None

        # V2
        thing_type = None
        thing = []
        for line in lines[1:]:
            m_header = re.search(r'^(#+) (.*)', line)
            if m_header:
                thing_type = f"header_${len(m_header.group(1))}"
                thing.append(m_header.group(2))






        for line in lines[1:]:
            if re.search(r'^---\s*$', line):
                content.append('<hr />')
                continue

            org_match = re.search(r'^(#\+\S+)', line)
            if org_match:
                if org_match.group(1) == "#+begin_src":
                    parts = line.split(" ")
                    content.append(f'<pre><code class="language-{parts[1]}">')
                    area_type = 'org_source_block'
                    continue
                elif org_match.group(1) == "#+end_src":
                    content.append('</code></pre>')
                    area_type = None
                    continue
                elif org_match.group(1) == "#+RESULTS:":
                    content.append('<h6 class="code-results">Results</h6>')
                    area_type = None
                    continue
                elif org_match.group(1) == "#+begin_example":
                    content.append(f'<pre><code class="language-txt">')
                    area_type = 'org_example_block'
                    continue
                elif org_match.group(1) == "#+end_example":
                    content.append('</code></pre>')
                    area_type = None
                    continue

            if area_type == 'org_source_block':
                content.append(line)
                continue

            if area_type == 'org_example_block':
                content.append(line)
                continue


            line = self.handle_escapes(line)

            line = self.handle_links(line)
            line = self.prep_line(line)
            # Don't output blank lines but don't skip here 
            # so they can be used for processing
            if line == '':
                offload = False

            # Don't strip, it messes with indent lines
            m_header = re.search(r'^(#+) (.*)', line)
            m_letter = re.search(r'^([a-zA-Z].*)', line)
            m_ul     = re.search(r'^-\s+(.*)', line)
            m_ol      = re.search(r'^\d+\.\s+(.*)', line)

            m_indent = re.search(r'^(\s+)(.*)', line) 
            m_empty  = re.search(r'^(\s*)$', line)

            # Setup the closing tags
            if (m_header or m_letter or m_ul) and area_type == 'ol':
                content.append('</ol>')
                area_type = None

            # Setup the closing tags
            if (m_header or m_letter or m_ol) and area_type == 'ul':
                content.append('</ul>')
                area_type = None

            # Set area type and opening tags
            if m_ul and area_type == None:
                content.append('<ul>')
                area_type = 'ul'
                offload = False

            if m_ol and area_type == None:
                content.append('<ol>')
                area_type = 'ol'
                offload = False

            # Make a header
            if m_header and area_type == None:
                level = len(m_header.group(1))
                assembler.append(f"<h{level}>{m_header.group(2).strip()}</h{level}>")
                offload = True

            # Start a paragraph
            elif m_letter and area_type == None:
                assembler.append(f'<p class="post-paragraph">{line}'.strip())
                area_type = "paragraph"
                offload = False

            # Middle of a paragraph
            elif m_letter and area_type == 'paragraph':
                assembler.append(line.strip())
                offload = False

            # Close a paragraph
            elif m_empty and area_type == 'paragraph':
                assembler[-1] = f"{assembler[-1]}</p>"
                area_type = None
                offload = True

            # Open li
            elif m_ul and area_type == 'ul':
                assembler.append(f'<li>{m_ul.group(1).strip()}')
                offload = False

            # Multiline li
            elif m_indent and area_type == 'ul':
                assembler.append(m_indent.group(2).strip())
                offload = False

            # Close li
            elif m_empty and area_type == 'ul' and len(assembler) > 0:
                assembler[-1] = f"{assembler[-1]}</li>"
                offload = True

            # Open ol_li
            elif m_ol and area_type == 'ol':
                assembler.append(f'<li>{m_ol.group(1).strip()}')
                offload = False

            # Multiline ol_li
            elif m_indent and area_type == 'ol':
                assembler.append(m_indent.group(2).strip())
                offload = False

            # Close ol_li
            elif m_empty and area_type == 'ol' and len(assembler) > 0:
                assembler[-1] = f"{assembler[-1]}</li>"
                offload = True

            # This cathes things that arne't empyt lines
            # e.g. images 
            elif not m_empty:
                content.append(line)

            # Offload the item and reset
            if offload == True:
                content.append(" ".join(assembler))
                assembler = []

        return content

    def process_div(self, lines):
        content = []
        attributes = []
        lines_to_process = []
        # Grab custom attributes
        for line in lines:
            parts = line.split(":::: :")
            if len(parts) > 1:
                details = parts[1].split(' ')
                items = " ".join(details[1:])
                attributes.append(f'{details[0]}="{items.strip()}"')
            else:
                lines_to_process.append(line)
        if len(attributes) > 0:
            content.append(f'<div {" ".join(attributes)}>')
        else:
            content.append('<div>')

        content.extend(self.process_content(lines_to_process))
        content.append('</div>')

        return content

    def process_html(self, lines):
        # just need to strip the content type
        return lines[1:]

    def process_notes(self, lines):
        content = ['<ul class="notes">']
        assembler = []
        offload = False
        for line in lines[1:]:
            line = self.handle_escapes(line)
            line = self.handle_links(line)
            if line == '':
                if len(assembler) > 0:
                    assembler[-1] = f'{assembler[-1].strip()}</li>'
                    offload = True
                else: 
                    pass
            elif line[0] == '-':
                assembler.append(f'<li class="notes__note">{line[2:].strip()}')
                offload = False
            else:
                assembler.append(line.strip())
                offload = False

            if offload == True:
                offload = False
                content.append(" ".join(assembler))
                assembler = []
        content.append('</ul>')
        return content

    def process_todo(self, lines):
        content = [
            '<div class="todo-list">', 
            '<h3 class="todo-list__header">TODO:</h3>',
            '<ul class="todo-list__list">']
        offload = False
        assembler = []
        for line in lines[1:]:
            m1 = re.search(r'^\[\s*\]\s+(.*)', line)
            if m1:
                assembler.append(f'<li class="todo-list__item-not-done">{m1.group(1).strip()}')
                offload = False
            else:
                m2 = re.search(r'^\[x]\s+(.*)', line)
                if m2:
                    assembler.append(f'<li class="todo-list__item-done">{m2.group(1).strip()}')
                    offload = False
                else:
                    if line != '':
                        assembler.append(line.strip())
                        offload = False
                    else:
                        if len(assembler) > 0:
                            assembler[-1] = f'{assembler[-1].strip()}</li>'
                            offload = True

            if offload == True:
                content.append(" ".join(assembler))
                assembler = []
                offload = False
        content.append('</ul>')
        content.append('</div>')
        return content

    def process_widget(self, lines):
        # NOTE: This is hard coded for now. Each widget
        # will have their own processing code. 
        content = [
            '''<details id="widget-example">''',
            '''<summary>Widget Example</summary>''',
            '''<ul>''',
            '''<li>Widgets are defined by JSON in the content</li>''',
            '''<li>It's up to the processor to handle them</li>''',
            '''</ul>''',
            '''</details>'''
        ]
        return content




