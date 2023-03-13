import {fromMarkdown} from 'https://esm.sh/mdast-util-from-markdown@1'

// Run this from deno with: 
// deno run input.js

const tree = fromMarkdown(`Inline markdown stuff

The *quick* and **brown** and _*fox*_ is here

`);

console.log(JSON.stringify(tree, null, 2));

