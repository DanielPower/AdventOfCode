import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

type Instruction = [string, number];

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);

let instructionCounter = 0;
let accumulator = 0;
const instructions: Instruction[] = [];

for await (const line of readLines(fileReader)) {
  const [operation, value] = line.split(" ");
  instructions.push([operation, parseInt(value, 10)]);
}

const completeInstructions = new Set();

while (true) {
  const [operation, value] = instructions[instructionCounter];
  console.log(operation, value);
  if (completeInstructions.has(instructionCounter)) {
    console.log(accumulator);
    break;
  }
  completeInstructions.add(instructionCounter);
  switch (operation) {
    case "nop": {
      instructionCounter += 1;
      break;
    }
    case "acc": {
      accumulator += value;
      instructionCounter += 1;
      break;
    }
    case "jmp": {
      instructionCounter += value;
    }
  }
}
