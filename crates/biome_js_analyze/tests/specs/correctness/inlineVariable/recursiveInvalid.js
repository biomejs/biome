// see https://github.com/rome/tools/issues/3697

const biomeKiller = () => {
    const fn = (callback) => {
      callback(fn);
    };
  };