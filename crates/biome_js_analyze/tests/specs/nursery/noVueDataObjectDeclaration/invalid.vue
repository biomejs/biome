<script>
// Vue.component with object `data`
Vue.component('some-comp', {
  data: {
    foo: 'bar'
  }
});

// app.component with object `data`
app.component('some-comp', {
  data: {
    foo: 'bar'
  }
});

// export default with object `data`
export default {
  data: {
    foo: 'bar'
  }
};

// export default with parenthesized object `data`
export default {
  data: /*a*/ (/*b*/{
    foo: 'bar'
  })
};

// createApp() then app.component with object `data`
const app = createApp(App)
app.component('some-comp', {
  data: {
    foo: 'bar'
  }
});

// app.component standalone with object `data`
app.component('some-comp', {
  data: {
    foo: 'bar'
  }
});

// export default with object `data`
export default {
  data: {
    foo: 'bar'
  }
};

// createApp with object `data`
createApp({
  data: {
    foo: 'bar'
  }
});

// createApp(...).mount with object `data`
createApp({
  data: {
    foo: 'bar'
  }
}).mount('#app');
</script>
