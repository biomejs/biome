const slotFn = isFirstMount
  ? ((i): Slot => ({ [CONTENT_SLOT]: i }))
  : i => wrapSlotExpr(newExprs[i]);
