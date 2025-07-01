<script>
// new Vue with function `data`
new Vue({
  data: function () {
    return {
      foo: 'bar'
    };
  }
});

// new Vue with method shorthand `data`
new Vue({
  ...data,
  data() {
    return {
      foo: 'bar'
    };
  }
});

// Vue.component with function `data`
Vue.component('some-comp', {
  data: function () {
    return {
      foo: 'bar'
    };
  }
});

// export default with function `data`
export default {
  data: function () {
    return {
      foo: 'bar'
    };
  }
};

// export default spreading other props
export default {
  ...foo
};

// export default shorthand `data`
export default {
  data
};

// export default arrow `data`
export default {
  data: () => {
    // no-op
  }
};

// export default method and methods/computed mix
export default {
  data() {},
  methods: {},
};

// createApp with function `data`
createApp({
  data: function () {
    return {
      foo: 'bar'
    };
  }
}).mount('#app');

// createApp with spread + method `data`
createApp({
  ...data,
  data() {
    return {
      foo: 'bar'
    };
  }
}).mount('#app');

// const app = createApp + app.component with function `data` (dot access)
const app = createApp(App);
app.component('some-comp', {
  data: function () {
    return {
      foo: 'bar'
    };
  }
});

// export default with data property on an object defined in methods
export default {
  methods: {
    foo() {
      const bar = {
        data: {}
      }
    }
  }
}
</script>
