type Exposed = {
  versions: Versions,
  ping: () => Promise<string>,
};

type Versions = {
  node: () => string,
  chrome: () => string,
  electron: () => string,
};

