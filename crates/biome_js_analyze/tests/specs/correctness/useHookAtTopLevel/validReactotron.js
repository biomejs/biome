/* should not generate diagnostics */

const reactotron = Reactotron.configure({ name: "biome_playground" })
    .use(reactotronRedux())
    .use(networking())
    .useReactNative({ overlay: true })
    .connect();
