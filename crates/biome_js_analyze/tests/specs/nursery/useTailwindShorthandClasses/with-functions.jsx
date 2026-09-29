const button = cn("px-2 py-2");

const card = tw.div`overflow-hidden text-ellipsis whitespace-nowrap`;

cva('', {
    variants: { size: { large: 'w-4 h-4', small: ['px-2 py-2'], medium: { 'mr-3 ml-3': active } } },
    compoundVariants: [{ size: 'large', class: 'px-2 py-2' }, { size: 'small', className: 'mr-3 ml-3' }],
});
tv({
    base: 'w-4 h-4',
    slots: { icon: 'px-2 py-2' },
    variants: { size: { large: 'w-4 h-4', small: { icon: 'mr-3 ml-3' } } },
    compoundVariants: [{ size: 'large', class: { icon: 'px-2 py-2' } }],
    compoundSlots: [{ slots: ['icon'], class: 'mr-3 ml-3' }],
});
cva('', { ...{ variants: { size: { large: 'w-4 h-4' } } } });
tv({ ...{ base: 'w-4 h-4' } });
tv({ ...(enabled && { base: 'w-4 h-4' }) });
tv(enabled ? { base: 'w-4 h-4' } : {});
tv(config = { ['base']: 'w-4 h-4' });
tv((config, { base: 'w-4 h-4' }));
cva('', enabled ? { variants: { size: { large: 'w-4 h-4' } } } : {});
