import type { MY_ATTRIBUTE} from './types';

export type T = {
  [MY_ATTRIBUTE]: number,
  [MY_ATTRIBUTE](): number,
  get [MY_ATTRIBUTE](): number,
  set [MY_ATTRIBUTE](x: number),
};

export declare class C {
    [MY_ATTRIBUTE]: number
    [MY_ATTRIBUTE](): number
    get [MY_ATTRIBUTE](): number
    set [MY_ATTRIBUTE](x: number)
};
