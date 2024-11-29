export type Project = {
  build?: () => boolean;
  run: (day: number, part: number) => Array<string>;
  container: string;
};

export type Year = {
  [key: string]: Project;
};
