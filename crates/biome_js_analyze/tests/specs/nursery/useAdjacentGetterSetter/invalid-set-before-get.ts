class User {
    get name() {
        return 'John Doe';
    }
    get #age() {
        return 30;
    }
    set #age(age) {}
    set name(value) {}
}

const user = {
    get age() {
        return 30;
    },
    get name() {
        return 'John Doe';
    },
    set name(value) {},
    set age(age) {}
};

type UserType = {
    set age(age: number);
    get name(): string;
    set name(value: string);
    get age(): number;
}

interface UserInterface {
    set name(value: string);
    get age(): number;
    set age(age: number);
    get name(): string;
}

declare module 'module' {
    export class User {
        get name(): string;
        set name(value: string);
        get #age(): number;
        set #age(age: number);
    }
}
