/* should generate diagnostics */
styles('w-4 h-4', {
    base: 'px-2 py-2',
    variants: {
        size: { sm: 'mr-3 ml-3', md: { 'w-4 h-4': active }, lg: { icon: 'px-2 py-2' } },
    },
    compoundVariants: [{ size: 'sm', class: 'mr-3 ml-3' }],
    compoundSlots: [{ slots: ['icon'], className: 'w-4 h-4' }],
});
cx('w-4 h-4', { 'px-2 py-2': active });
