import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

type Operation = "nop" | "acc" | "jmp";

const instructions: [Operation, number][] = [];
const completeInstructions = new Set();
let instructionCounter = 0;
let accumulator = 0;

const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
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
  })[operation]();
}
