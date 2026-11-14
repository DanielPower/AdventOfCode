// {{year}} day {{day}}, part {{part}}: https://adventofcode.com/{{year}}/day/{{day}}
// (Bun.stdin.text() fails on piped stdin in Bun 1.0.15.)
const input = await new Response(Bun.stdin.stream()).text();
const lines = input.trimEnd().split("\n");

console.log(lines.length);
