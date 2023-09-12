export default class {
    constructor(
        readonly p: unknown,
        protected property: unknown,
        public camelCase: unknown,
        private _privateProperty: unknown,
    ) {}
}