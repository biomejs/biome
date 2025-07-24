<>
  {/* Test cases for whitespace handling */}
  <p className="text-sm font-bold            ">hello world</p>
  <p className="            text-sm font-bold">hello world</p>  
  <p className="text-sm font-bold            other-class">hello world</p>
  <p className="   ">empty with spaces</p>
  <p className="">empty without spaces</p>
  <p className="text-sm    font-bold">multiple spaces between</p>
  <p className="  text-sm  font-bold  ">spaces everywhere</p>
  
  {/* Template literal cases */}
  <p className={`text-sm font-bold            `}>template literal trailing</p>
  <p className={`            text-sm font-bold`}>template literal leading</p>
  <p className={`  text-sm  font-bold  `}>template literal both</p>
</>