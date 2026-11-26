/* should generate diagnostics */
import { defineComponent } from "vue";

export const NoEmitsOption = defineComponent({
	methods: {
		close() {
			this.$emit("close");
		},
	},
});

export const ArrayEmits = defineComponent({
	emits: ["change"],
	methods: {
		close() {
			this.$emit("close");
		},
	},
});

export const ObjectEmits = defineComponent({
	emits: { change: null, submit: (value) => true },
	methods: {
		close() {
			this.$emit("close");
		},
	},
});

export const EmptyEmits = defineComponent({
	emits: [],
	methods: {
		close() {
			this.$emit("close");
		},
	},
});

export const SetupContext = defineComponent({
	emits: ["change"],
	setup(props, context) {
		context.emit("close");
	},
});

export const SetupDestructured = defineComponent({
	emits: ["change"],
	setup(props, { emit }) {
		emit("close");
	},
});

export const SetupRenamed = defineComponent({
	emits: ["change"],
	setup(props, { emit: fire }) {
		fire("open");
		fire("close");
	},
});

export const Parent = defineComponent({
	emits: ["parent"],
	components: {
		Child: defineComponent({
			emits: ["child"],
			methods: {
				notify() {
					this.$emit("parent");
				},
			},
		}),
	},
	methods: {
		notify() {
			this.$emit("child");
		},
	},
});

export const OptionalCall = defineComponent({
	methods: {
		close() {
			this?.$emit?.("close");
		},
	},
});
