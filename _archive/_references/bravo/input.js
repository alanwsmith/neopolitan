import {fromMarkdown} from 'https://esm.sh/mdast-util-from-markdown@1'

// Run this from deno with: 
// deno run input.js

const tree = fromMarkdown(`# This is an H1

## An H2

With some text

### And an H3

And more text`);

console.log(JSON.stringify(tree, null, 2));

