interface Greeter {
	message: any;
}

interface Greeter2 {
	message: Array<any>;
}

interface Greeter3 {
	message: any[];
}

interface Greeter4 {
	message: Array<Array<any>>;
}

interface Greeter5 {
	message: Array<any[]>;
}

interface Qux5 { (...args: any): void; }

interface Grault5 { new (...args: any): void; }

interface Garply5 { f(...args: any): void; }

type obj = {
	message: any;
}

type obj2 = {
	message: Array<any>;
}

type obj3 = {
	message: any[];
}

type obj4 = {
	message: Array<Array<any>>;
}

type obj5 = {
	message: Array<any[]>;
}

type obj6 = {
	message: string | any;
}

type obj7 = {
	message: string | Array<any>;
}

type obj8 = {
	message: string | any[];
}

type obj9 = {
	message: string | Array<Array<any>>;
}

type obj10 = {
	message: string | Array<any[]>;
}

type obj11 = {
	message: string & any;
}

type obj12 = {
	message: string & Array<any>;
}

type obj13 = {
	message: string & any[];
}

type obj14 = {
	message: string & Array<Array<any>>;
}

type obj15 = {
	message: string & Array<any[]>;
}

type Any = any;

type Fred5 = (...args: any) => void;

type Corge5 = new (...args: any) => void;
