// {{year}} day {{day}}, part {{part}}: https://adventofcode.com/{{year}}/day/{{day}}
const input = await new Response(Deno.stdin.readable).text();
const lines = input.trimEnd().split("\n");

console.log(lines.length);
