<svg width="200" height="150"
    xmlns="http://www.w3.org/2000/svg"
    xmlns:xlink="http://www.w3.org/1999/xlink">

  <defs>
      <circle id="myCircle" cx="0" cy="0" r="30" fill="orange" />
  </defs>

  <!-- xlink:href should be parsed as a single attribute -->
  <use xlink:href="#myCircle" x="50" y="50" />
  <use xlink:href="#myCircle" x="150" y="100" fill="steelblue" />

</svg>
