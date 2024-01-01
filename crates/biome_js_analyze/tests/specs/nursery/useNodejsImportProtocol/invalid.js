import assert from "assert";
import assert_strict from "assert/strict";
import async_hooks from "async_hooks";
import buffer from "buffer";
import child_process from "child_process";
import cluster from "cluster";
import console from "console";
import constants from "constants";
import crypto from "crypto";
import dgram from "dgram";
import diagnostics_channel from "diagnostics_channel";
import dns from "dns";
import dns_promises from "dns/promises";
import domain from "domain";
import events from "events";
import fs from "fs";
import fs_promises from "fs/promises";
import http from "http";
import http2 from "http2";
import https from "https";
import inspector from "inspector";
import inspector_promises from "inspector/promises";
import module from "module";
import net from "net";
import os from "os";
import path from "path";
import path_posix from "path/posix";
import path_win32 from "path/win32";
import perf_hooks from "perf_hooks";
import process from "process";
import punycode from "punycode";
import querystring from "querystring";
import readline from "readline";
import readline_promises from "readline/promises";
import repl from "repl";
import stream from "stream";
import stream_consumers from "stream/consumers";
import stream_promises from "stream/promises";
import stream_web from "stream/web";
import string_decoder from "string_decoder";
import sys from "sys";
import timers from "timers";
import timers_promises from "timers/promises";
import tls from "tls";
import trace_events from "trace_events";
import tty from "tty";
import url from "url";
import util from "util";
import util_types from "util/types";
import v8 from "v8";
import vm from "vm";
import wasi from "wasi";
import worker_threads from "worker_threads";
import zlib from "zlib";

// check for require and import
require("fs");
import("fs");

// Use same quote style
import assert from "assert";
import assert from 'assert';

// Keep comments
import assert from /*0*/ "assert" /*b*/;
