import {Head} from 'next/document';

export class Blah extends Head {
    render() {
      return (
        <div>
          <h1>Hello title</h1>
          <script src='https://polyfill.io/v3/polyfill.min.js?features=WeakSet%2CPromise%2CPromise.prototype.finally%2Ces2015%2Ces5%2Ces6'></script>
          <script src='https://polyfill.io/v3/polyfill.min.js?features=Array.prototype.copyWithin'></script>
          <script src='https://polyfill.io/v3/polyfill.min.js?features=Object.fromEntries'></script>
          <script src='https://polyfill.io/v3/polyfill.min.js?features=AbortController,Object.fromEntries'></script>
        </div>
      );
    }
}
