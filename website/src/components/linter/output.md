```js
  main.tsx:9:3 lint/nursery/noUnreachable ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  
    ✖ This code will never be reached ...
    
       7 │   }
       8 │ 
     > 9 │   return 20;
         │   ^^^^^^^^^^
      10 │ }
    
    ℹ ... because either this statement will return from the function ...
    
      1 │ function test(callback) {
      2 │   try {
    > 3 │     return callback();
        │     ^^^^^^^^^^^^^^^^^^
      4 │   } catch (e) {
      5 │     console.log(e);
    
    ℹ ... or this statement will throw an exception beforehand
    
      4 │   } catch (e) {
      5 │     console.log(e);
    > 6 │     throw e;
        │     ^^^^^^^^
      7 │   }
      8 │ 
    
    ```