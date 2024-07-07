function fn(member) {
  fn(<>{member.expression}</>);
  fn(<>{member.expression()}</>);
  (<>{1}</>).toString();
}
