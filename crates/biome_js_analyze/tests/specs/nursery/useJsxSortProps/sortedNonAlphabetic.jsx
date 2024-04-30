/* should not generate diagnostics */
<Hello
  name="John"
  tel={5555555}
  classes={{
    greetings: classes.greetings,
  }}
/>;

<Hello
  tel={5555555}
  name="John"
  classes={{
    greetings: classes.greetings,
  }}
  validate
  active
	onClick={this._handleClick}
	onChange={this._handleChange}
/>;
