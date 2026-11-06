const Button = styled.button`
  display:inline-flex;
  color:red;
  width : 100px;
  padding:4px 8px;
  border-left:1px solid blue;
  overflow:hidden;
  &:hover{
    background:green;
  }
  .icon:hover &{
    margin:-4px;
  }
  @media (min-width:768px){
    display:none;
  }
  &.active{
    color : red ;
    font-weight:bold;
  }
`;

const Card = styled.div`
  position:relative;
  margin:8px auto;
  font: 16px/1.5 sans-serif;
  transition: opacity 200ms ease-in-out;
  background:url("card.png") no-repeat;
  .title-primary{
    color:black;
  }
  > .button + .button{
    margin-left:4px;
  }
`;
