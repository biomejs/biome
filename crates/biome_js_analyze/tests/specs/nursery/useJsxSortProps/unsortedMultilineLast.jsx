<Hello
  name="John"
  classes={{
    greetings: classes.greetings,
  }}
  tel={5555555}
/>;

<Hello
  name="John"
  tel={5555555}
  active
  validate
  classes={{
    greetings: classes.greetings,
  }}
	onChange={this._handleChange}
	onClick={this._handleClick}
/>;

<Hello
  name="John"
  tel={5555555}
  active
  validate
  onChange={this._handleChange}
  classes={{
    greetings: classes.greetings,
  }}
  onClick={this._handleClick}
/>;
