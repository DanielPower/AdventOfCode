import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

const instructions: [string, number][] = [];

const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
  if (!line) {
    break;
  }
  const [operation, value] = line.split(" ");
  instructions.push([operation, parseInt(value, 10)]);
}

const testForLoop = () => {
  let instructionCounter = 0;
  let accumulator = 0;
  const completeInstructions = new Set();
  while (true) {
    if (instructionCounter >= instructions.length) {
      return accumulator;
    }
    if (completeInstructions.has(instructionCounter)) {
      return false;
    }
    completeInstructions.add(instructionCounter);
    const [operation, value] = instructions[instructionCounter];
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
        break;
      }
    }
  }
};

instructions.forEach(([operation, value], index) => {
  let tryOperation;
  if (operation === "jmp") {
    tryOperation = "nop";
  } else if (operation === "nop") {
    tryOperation = "jmp";
  }

  if (tryOperation) {
    instructions[index] = [tryOperation, value];
    const result = testForLoop();
    if (result) {
      console.log(result);
    }
    instructions[index] = [operation, value];
  }
});
