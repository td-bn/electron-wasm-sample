type Exposed = {
  versions: Versions,
  ping: () => Promise<string>,
  greet: (s: string) => Promise<string>,
};

type Versions = {
  node: () => string,
  chrome: () => string,
  electron: () => string,
};

