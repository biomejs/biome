/* should not generate diagnostics */
<Hello name="John" tel={5555555} active validate />;
<Hello name="John" active {...props} tel={5555555} validate />;
<Hello name="John" active {...props} tel={5555555} multi validate />;
<Hello name="John" active onChange={this._handleChange} {...props} tel={5555555} multi validate onClick={this._handleClick} />;
