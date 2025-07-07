/* should not generate diagnostics */
export default {
    props: {
        // This is not a vue file, so it should not be flagged
        ref: String,
        key: String,
    }
};
