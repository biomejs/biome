import x = require("./test");
namespace a.b {}
import y = a;
import z = a.b;
import type A = require("./a");
export import n = a;
