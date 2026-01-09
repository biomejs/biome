Basic (3 backticks):
```
code
```

Longer fence (5 backticks):
`````
code with ``` inside
`````

Tildes:
~~~
code
~~~

Mixed (should not close):
```
code
~~~
still code
```

Indented closing (valid):
```
code
   ```

Short closing (invalid - treated as content):
````
code
```
still code
````

Indented opening (stripped from content):
  ```
  code line
  ```
