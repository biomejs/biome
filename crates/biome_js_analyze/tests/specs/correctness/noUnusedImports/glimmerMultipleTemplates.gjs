/* should not generate diagnostics - components used across multiple templates */
import Button from './Button';
import Card from './Card';
import Dialog from './Dialog';

export const ButtonComponent = <template><Button /></template>;
export const CardComponent = <template><Card /></template>;
export const DialogComponent = <template><Dialog /></template>;
