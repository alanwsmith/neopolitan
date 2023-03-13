import {fromMarkdown} from 'https://esm.sh/mdast-util-from-markdown@1'

// Run this from deno with: 
// deno run input.js

const tree = fromMarkdown(`###### H6

Here is a list

- item a
- item b
- item c

that's it
`);

console.log(JSON.stringify(tree, null, 2));

