<script>
export default {
    props: ['foo'],
    computed: {
        bar() {}
    },
    data: () => ({
        dat: null
    }),
    methods: {
        _foo () {},
        test () {
        }
    },
    setup() {
        return {
            _bar: () => {}
        }
    }
};
</script>
