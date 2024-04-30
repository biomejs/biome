/* should not generate diagnostics */
<Hello
  name="John"
  tel={5555555}
  classes={{
    greetings: classes.greetings,
  }}
/>;

<Hello
  name="John"
  tel={5555555}
  classes={{
    greetings: classes.greetings,
  }}
  active
  validate
	onChange={this._handleChange}
	onClick={this._handleClick}
/>;
