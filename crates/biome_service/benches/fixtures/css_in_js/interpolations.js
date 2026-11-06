const Button = styled.button`
  display:inline-flex;
  color:${ (props)=>props.color };
  width : ${({width})=>width}px;
  padding:${({ size }) => size * 2}px ${({ size }) => size * 4}px;
  border-${side}:1px solid ${({ theme }) => theme.border};
  ${truncate};
  &:hover{
    background:${({ theme }) => theme.hover};
  }
  ${Icon}:hover &{
    margin:-${gap}px;
  }
  @media (min-width:${breakpoint}px){
    display:none;
  }
  ${(props) => props.active && css`
    color : red ;
    font-weight:${weight};
  `};
`;

const Card = styled.div`
  position:relative;
  margin:${({ gap }) => gap}px auto;
  font: ${size}px/${lineHeight} ${family};
  transition: opacity ${duration}ms ease-in-out;
  background:url(${image("card.png")}) no-repeat;
  .title-${variant}{
    color:${({ theme }) => theme.title};
  }
  > ${Button} + ${Button}{
    margin-left:${spacing}px;
  }
`;
