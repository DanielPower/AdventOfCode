import { Year } from "../types.ts";

export const year: Year = {
  python: {
    build: () => true,
    run: (root: string, day: number, part: number) => {
      const path = `${root}/${day}/${part}.py`;
      console.log(path);
      const command = new Deno.Command("python", {
        args: [`${day}/part${part}.py`],
      });
      const output = command.outputSync();
      console.log(output.stdout);
      return "";
    },
  },
};
