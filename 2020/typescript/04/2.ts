import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

const hex = [
  "0",
  "1",
  "2",
  "3",
  "4",
  "5",
  "6",
  "7",
  "8",
  "9",
  "a",
  "b",
  "c",
  "d",
  "e",
  "f",
];
const num = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const clr = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
const fieldValidators: [string, (value: string) => boolean][] = [
  ["byr", (value) => parseInt(value) >= 1920 && parseInt(value) <= 2002],
  ["iyr", (value) => parseInt(value) >= 2010 && parseInt(value) <= 2020],
  [
    "eyr",
    (value) =>
      value.length === 4 && parseInt(value) >= 2020 && parseInt(value) <= 2030,
  ],
  [
    "hgt",
    (value) => {
      if (value.slice(-2) === "in") {
        return parseInt(value) >= 59 && parseInt(value) <= 76;
      } else if (value.slice(-2) === "cm") {
        return parseInt(value) >= 150 && parseInt(value) <= 193;
      }
      return false;
    },
  ],
  [
    "hcl",
    (value) =>
      value.length === 7 &&
      value[0] === "#" &&
      !value
        .slice(1)
        .split("")
        .find((char) => !hex.includes(char)),
  ],
  ["ecl", (value) => clr.includes(value)],
  [
    "pid",
    (value) =>
      value.length === 9 &&
      !value.split("").find((char) => !num.includes(char)),
  ],
];

let passport: { [key: string]: string } = {};
let validPassports = 0;
const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
  if (line === "") {
    if (
      !fieldValidators.find(([key, validator]) => {
        return !passport[key] || !validator(passport[key]);
      })
    ) {
      validPassports += 1;
    }
    passport = {};
  } else {
    line.split(" ").forEach((field) => {
      const key = field.split("").splice(0, 3).join("");
      const value = field.split("").splice(4).join("");
      passport[key] = value;
    });
  }
}
console.log(validPassports);
