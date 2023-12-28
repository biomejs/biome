app.get("/", (req, res): void => {
    res.send("Hello World!");
})


export class Thing implements OtherThing {
    do: (type: Type) => Provider<Prop> = memoize(
        (type: ObjectType): Provider<Opts> => {}
    );
}

// Issue https://github.com/rome/tools/issues/2756
export class Task {
    args: any[];

    constructor(
        public script: string,
        public duration: number,
        public threadCount: number,
        ...args: any[]
    ) {
        this.args = args;
    }
}

// Issue https://github.com/biomejs/biome/issues/1171
users.map((user: User): User => {
  ; // comment
})

users.map((user: User): User => {
  // comment
})
