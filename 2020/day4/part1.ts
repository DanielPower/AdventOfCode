import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

const requiredFields = [
  "byr",
  "iyr",
  "eyr",
  "hgt",
  "hcl",
  "ecl",
  "pid",
];

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);
let passport: string[] = [];
let validPassports = 0;
for await (const line of readLines(fileReader)) {
  if (line === "") {
    const fieldNames = new Set();
    passport.forEach((field) => {
      fieldNames.add(field.slice(0, 3));
    });
    if (!requiredFields.find((requiredField) => !fieldNames.has(requiredField))) {
      validPassports += 1;
    }
    console.log(fieldNames);
    passport = [];
  } else {
    passport = [...passport, ...line.split(" ")];
  }
}
console.log(validPassports);
