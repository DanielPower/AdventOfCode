export type Project = {
  build?: (
    day: number,
    part: number,
  ) => {
    args: Array<string>;
    cwd?: string;
  };
  run: (
    day: number,
    part: number,
  ) => {
    args: Array<string>;
    cwd?: string;
  };
  exists: (root: string, day: number, part: number) => boolean;
  container: string;
};

export type Year = {
  [key: string]: Project;
};
