import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

type Operation = "nop" | "acc" | "jmp";

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);
const instructions: [Operation, number][] = [];
const completeInstructions = new Set();
let instructionCounter = 0;
let accumulator = 0;

for await (const line of readLines(fileReader)) {
  const [operation, value] = line.split(" ");
  instructions.push([operation as Operation, parseInt(value, 10)]);
}

while (true) {
  const [operation, value] = instructions[instructionCounter];
  if (completeInstructions.has(instructionCounter)) {
    console.log(accumulator);
    break;
  }
  completeInstructions.add(instructionCounter);
  ({
    nop: (): void => {
      instructionCounter += 1;
    },
    acc: (): void => {
      accumulator += value;
      instructionCounter += 1;
    },
    jmp: (): void => {
      instructionCounter += value;
    },
  }[operation]());
}
