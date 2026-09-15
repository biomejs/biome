/* should not generate diagnostics */
const value = <div className={[
    'w-4 h-4' === state ? 'size-4' : 'p-2',
    'w-4 h-4' && 'size-4',
    { 'size-4': 'w-4 h-4' },
    lookup['w-4 h-4'],
    lookup(`w-4 h-4`),
    (() => 'w-4 h-4')(),
]} />;
clsx({ 'size-4': 'w-4 h-4' });
cva('', {
    variants: { size: { 'w-4 h-4': 'size-4', small: { 'p-2': 'w-4 h-4' } } },
    defaultVariants: { size: 'w-4 h-4' },
    compoundVariants: [{ size: 'w-4 h-4', class: 'size-4' }],
});
tv({
    slots: { 'w-4 h-4': 'size-4' },
    variants: { size: { 'w-4 h-4': { 'px-2 py-2': 'size-4' } } },
    defaultVariants: { size: 'w-4 h-4' },
    compoundVariants: [{ size: 'w-4 h-4', class: 'size-4' }],
    compoundSlots: [{ slots: ['w-4 h-4'], class: 'size-4' }],
});
tv({ base: 'w-4 h-4' } ? {} : {});
tv({ ...({ base: 'w-4 h-4' } && {}) });
tv(({ base: 'w-4 h-4' }, {}));
