let count:number=$state(1)
const doubled=$derived(count*2)
export const read=($offset:number)=>doubled+$offset
