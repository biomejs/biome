const slotFn = isFirstMount
  ? ({ item }: Filter): Slot => item
  : other;
