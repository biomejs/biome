const dirname = import.meta.dirname;
const filename = import.meta.filename;
const dirname_ = { __dirname: import.meta.dirname };
const filename_ = { __filename: import.meta.filename };
const dirname__ = { dirname: import.meta.dirname, foo: "foo" };
const filename__ = { filename: import.meta.filename, foo: "foo" };

if (import.meta.dirname.startsWith("/project/src/")) {}
if (import.meta.filename.endsWith(".js")) {}
