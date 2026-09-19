// should generate diagnostics
const button = cn("hover:text-red-500/80");

const card = tw.div`bg-blue-500`;

cva('', {
    variants: { size: { large: 'bg-pink-500', small: ['hover:text-red-500/80'], medium: { 'border-slate-950': active } } },
    compoundVariants: [{ size: 'large', class: 'hover:text-red-500/80' }, { size: 'small', className: 'border-slate-950' }],
});
tv({
    base: 'bg-pink-500',
    slots: { icon: 'hover:text-red-500/80' },
    variants: { size: { large: 'bg-pink-500', small: { icon: 'border-slate-950' } } },
    compoundVariants: [{ size: 'large', class: { icon: 'hover:text-red-500/80' } }],
    compoundSlots: [{ slots: ['icon'], class: 'border-slate-950' }],
});
cva('', { ...{ variants: { size: { large: 'bg-pink-500' } } } });
tv({ ...{ base: 'bg-pink-500' } });
tv({ ...(enabled && { base: 'bg-pink-500' }) });
tv(enabled ? { base: 'bg-pink-500' } : {});
tv(config = { ['base']: 'bg-pink-500' });
tv((config, { base: 'bg-pink-500' }));
cva('', enabled ? { variants: { size: { large: 'bg-pink-500' } } } : {});
