/* should not generate diagnostics */
import "./valid";
import "react";
import "@testing-library/react";
import("./valid");
import("react");
import("@testing-library/react");
require("./valid");
require("react");
require("@testing-library/react");

import "node:assert";
require("node:assert");

import "bun:test";

import Button from "@mui/material/Button";
import { fontFamily } from "tailwindcss/defaultTheme";

import "peer-dep";
import "optional-dep";

import "my-package"

import "@/internal";
import "#internal";

// import from `@types/jest`
import type * as jest from "lodash";

import "bun";

// NodeJS builtin
import "fs";
import "os";
import "path";
import "process";

import "dep.js"