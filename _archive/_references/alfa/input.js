import {fromMarkdown} from 'https://esm.sh/mdast-util-from-markdown@1'

// Run this from deno with: 
// deno run input.js

const tree = fromMarkdown(`# This is an H1

The rest of this content is filled out
with different tests. This is a paragraph
that spans multiple lines

And another paragraph here`);

console.log(JSON.stringify(tree, null, 2));

