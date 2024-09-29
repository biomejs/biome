const a = 1;
const userName = 'testUser';
const count = 10;
const nonSecret = "hello world"
const nonSecretLong = "hello world, this is a looong string which I needed to create for some reason"
const dbUrl = `postgres://user:${process.env.DB_PASSWORD}@example.com:5432/dbname`;
const NOT_A_SECRET = "I'm not a secret, I think";
const webpackFriendlyConsole = require('./config/webpack/webpackFriendlyConsole');
const NOT_A_SECRET_TEMPLATE = `A template that isn't a secret. ${1+1} = 2`;
const CSS_CLASSNAME = "hey-it-s-a-css-class-not-a-secret and-neither-this-one";
const BASE64_CHARS = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
const NAMESPACE_CLASSNAME = 'Validation.JSONSchemaValidationUtilsImplFactory';

// Ignored secrets (not actually a secret, ignored via config)
const NOT_A_SECRET_IN_VAULT = {
  NOT_A_SECRET: "ZWVTjPQSdhwRgl204Hc51YCsritMIzn8B=/p9UyeX7xu6KkAGqfm3FJ+oObLDNEva"
};
const NOT_A_SECRET_VAR = "ZWVTjPQSdhwRgl204Hc51YCsritMIzn8B=/p9UyeX7xu6KkAGqfm3FJ+oObLDNEva"; // Ignored via variable name
class A {
  constructor(){
    this.secret = "ZWVTjPQSdhwRgl204Hc51YCsritMIzn8B=/p9UyeX7xu6KkAGqfm3FJ+oObLDNEva"; // Ignored via class field
  }
}


// From user tests
const codeCheck = "\nconst DEFAULT_CONFIG = /* @__PURE__ */ bare.Config({})\n";
const otpCheck = 'Verify OTP Google Mobile Authenticator (2FAS)'
const bitcoinString = {
	key: "0 USD,,. for {bitlocus|string}.",
};
const textString = {
    key: 'Verifying takes 15 approved the following 3.'
};
const facebookAndAwsString = {
    key: 'facebook.com |console.aws.amazon.com'
};
const IsoString = {
    key: 'ISO-27001 information , GDPR'
};