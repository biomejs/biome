type X = {
    'new'(): string;
};
const a: X = {
    new: () => '123',
};
console.log(a.new());