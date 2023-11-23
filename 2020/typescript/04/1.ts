import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

const requiredFields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

let passport: string[] = [];
let validPassports = 0;
const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
  if (line === "") {
    const fieldNames = new Set();
    passport.forEach((field) => {
      fieldNames.add(field.slice(0, 3));
    });
    if (
      !requiredFields.find((requiredField) => !fieldNames.has(requiredField))
    ) {
      validPassports += 1;
    }
    passport = [];
  } else {
    passport = [...passport, ...line.split(" ")];
  }
}
console.log(validPassports);
