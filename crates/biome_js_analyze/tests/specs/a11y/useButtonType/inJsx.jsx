// invalid
<>
    <button>do something</button>
    <button type="bar">do something</button>
    <button type>do something</button>
    <button/>
    <button type="bar"/>
    <button type/>
    <button onClick={null}>test</button>
    <button onClick={null}/>
</>


// valid
<>
    <button type="button">do something</button>
    <button type={dynamic_value}>do something</button>
    <button type="button"/>
    <button type={dynamic_value}/>
    <button {...props}></button>
</>