// should generate diagnostics
import { component$ } from "@builder.io/qwik";

export default component$(() => {
  const items = ["apple", "banana", "cherry"];

  return (
    <div>
      {/* Missing key in map */}
      {items.map(item => <li>{item}</li>)}
      
      {/* Missing key in forEach */}
      {items.forEach(item => <span>{item}</span>)}
      
      {/* Missing key in filter + map */}
      {items.filter(item => item.length > 5).map(item => <div>{item}</div>)}
      
      {/* Missing key in nested map */}
      {items.map(category => 
        category.split('').map(letter => <span>{letter}</span>)
      )}
      
      {/* Missing key in object map */}
      {Object.keys({a: 1, b: 2}).map(key => <div>{key}</div>)}
      
      {/* Missing key in array literal map */}
      {[1, 2, 3].map(num => <p>{num}</p>)}
    </div>
  );
}); 