// Removing the last import should add `export {}` to preserve module context

import {foo as _foo} from "foo";

function foo():number {
  return 0;
}

declare module "react" {
  interface ImgHTMLAttributes<T> extends HTMLAttributes<T> {
    fetchpriority?: "high" | "low" | "auto";
  }
}
