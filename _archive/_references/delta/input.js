import {fromMarkdown} from 'https://esm.sh/mdast-util-from-markdown@1'

// Run this from deno with: 
// deno run input.js

const tree = fromMarkdown(`##### An H5

This is \`inline code\` to test`);

console.log(JSON.stringify(tree, null, 2));

