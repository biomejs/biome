<script>
export default {
    ...foo(),
    props: {
        ...foo(),
        foo: String
    },
    computed: {
        ...mapGetters({
            test: 'getTest'
        }),
        bar: {
            get () {}
        }
    }
}
</script>