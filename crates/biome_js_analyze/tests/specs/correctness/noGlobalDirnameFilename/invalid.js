const dirname = __dirname;
const filename = __filename;
const dirname_ = { __dirname };
const filename_ = { __filename };
const dirname__ = { dirname: __dirname, foo: "foo" };
const filename__ = { filename: __filename, foo: "foo" };

if (__dirname.startsWith("/project/src/")) {}
if (__filename.endsWith(".js")) {}
