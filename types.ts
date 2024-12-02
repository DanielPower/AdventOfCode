export type Project = {
  build?: (
    day: string,
    part: string,
  ) => {
    args: Array<string>;
    cwd?: string;
  };
  run: (
    day: string,
    part: string,
  ) => {
    args: Array<string>;
    cwd?: string;
  };
  exists: (root: string, day: string, part: string) => boolean;
  container: string;
};

export type Year = {
  [key: string]: Project;
};
