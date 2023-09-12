import x from "y";
x = 1;

import y from "y";
[y] = 1;

import z from "y";
({ z } = 1);

import a from "y";
[...a] = 1;

import b from "y";
({ ...b } = 1);

import c from "y";
for (c in y) {};

import d from "y";
d += 1;

import * as e from "y";
e = 1;

import { f } from "y";
f = 1;
f = 2;

import {xx} from 'y';
xx=1;

import xxx, * as yyy from "d"
xxx = 4;
yyy = 4;