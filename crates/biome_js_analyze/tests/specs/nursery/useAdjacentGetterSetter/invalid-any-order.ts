class User {
    get name() {
        return 'John Doe';
    }
    get #age() {
        return 30;
    }
    set name(value) {}
    set #age(age) {}
}

const user = {
    get age() {
        return 30;
    },
    get name() {
        return 'John Doe';
    },
    set age(age) {},
    set name(value) {}
};

type UserType = {
    set age(age: number);
    set name(value: string);
    get age(): number;
    get name(): string;
}

interface UserInterface {
    set name(value: string);
    set age(age: number);
    get name(): string;
    get age(): number;
}

declare module 'module' {
    export class User {
        get name(): string;
        set #age(age: number);
        set name(value: string);
        get #age(): number;
    }
}
