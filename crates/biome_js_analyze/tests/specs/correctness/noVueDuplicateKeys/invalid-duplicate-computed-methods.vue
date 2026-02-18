<script>
export default {
    computed: {
        count() {
            return this.value * 2;
        }
    },
    methods: {
        count() {
            this.value++;
        }
    }
};
</script>