import {fromMarkdown} from 'https://esm.sh/mdast-util-from-markdown@1'

// Run this from deno with: 
// deno run input.js

const tree = fromMarkdown(`#### An H4

\`\`\`rust
fn main() {
  println!("Some rust code");
}
\`\`\`
`);

console.log(JSON.stringify(tree, null, 2));

