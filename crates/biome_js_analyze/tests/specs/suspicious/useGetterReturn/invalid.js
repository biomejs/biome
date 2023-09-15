var foo = {
    get bar() {}
};

var foo = {
    get bar(){
        if(baz) {
            return true;
        }
    }
};

var foo = {
    get bar() {
        ~function () {
            return true;
        }
    }
};

var foo = {
    get bar() {
        return;
    }
};

class Foo {
    get bar() {}
}

class Foo {
    get bar(){
        if(baz) {
            return true;
        }
    }
}

class Foo {
    get bar() {
        ~function () {
            return true;
        }
    }
}

class Foo {
    get bar() {
        return;
    }
}

class Foo {
    get bar(){
        try {
            return foo();
        } catch {} finally {}
    }
}

class Foo {
    get bar(){
        switch (this.n) {
            case 0:
                return 0;
            case 1:
            case 2:
                break;
        }
    }
}

var foo = {
    get bar(){
        if(baz) {
            return true;
        } else {
			false;
		}
    }
};

var foo = {
    get bar(){
        if(baz) {
           true;
        } else {
			return false;
		}
    }
};

var foo = {
    get bar(){
        for (;;) {
			break;
		}
		while (false) {
			return true;
		}
    }
};

var foo = {
    get bar(){
        do {
            if (bar()) {
                return 0;
            }
        } while(foo());
        for (x in [1, 2]) {
            if (x == 0) {
                return 0;
            }
        }
    }
};

var foo = {
    get bar(){
        if(baz) {
            if (foo) {
               return true;
            }
        } else {
            return true;
        }
    }
};
