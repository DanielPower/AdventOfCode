# {{year}} day {{day}}, part {{part}}: https://adventofcode.com/{{year}}/day/{{day}}
import sys


def solve(text: str) -> int:
    lines = text.splitlines()
    return len(lines)


print(solve(sys.stdin.read()))
