/* should generate diagnostics */
import { query } from "./_generated/server";
import { mutation } from "@/convex/_generated/server";
import { action } from "../lib/functions";

export const list = query(async (ctx) => {});

export const send = mutation(async (ctx) => {});

export const run = action(async (ctx) => {});
