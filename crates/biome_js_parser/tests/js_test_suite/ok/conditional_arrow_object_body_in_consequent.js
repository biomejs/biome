const slotFn = isFirstMount
  ? i => ({ [CONTENT_SLOT]: i })
  : i => wrapSlotExpr(newExprs[i]);
