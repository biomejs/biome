const self = this, v = 0, /*u*/ u = 2, self2 = this;

function f() {
    // assignment comment
    const self = this;
    return () => {
        /*a*/self/*b*/.g();
    }
}

function f() {
    let self = this;
    return () => {
        self.g();
    }
}

function f() {
    var self;
    self = this;
    self = this;
    return () => {
        self.g();
    }
}


// https://github.com/biomejs/biome/issues/1633
class FetchServersMetrics extends React.Component {
    _fetchServersMetrics() {
      const { onStatusChange, appIds, filters, time } = this.props;
      const component = this;

      fetchServers(appIds, time)
        .then(servers => {
          const data = isEmpty(filters) ? servers : applyFilters(servers, filters);
          component.setState({
            timestamp: new Date().getTime(),
            isLoading: false,
            data: data.filter(identity),
          });
        })
        .catch((err) => {
          component.setState({ serverError: throwOrResolve(err) });
        })
        .then(() => component.setState({ isLoading: false }));
    }

    render() {
      const { children } = this.props;
      return children(this._fetchServerMetrics);
    }
  }
