import {fromMarkdown} from 'https://esm.sh/mdast-util-from-markdown@1'

// Run this from deno with: 
// deno run input.js

const tree = fromMarkdown(`This is an ordered list

1. Item alfa
2. Item bravo
3. Item charlie
`);

console.log(JSON.stringify(tree, null, 2));

