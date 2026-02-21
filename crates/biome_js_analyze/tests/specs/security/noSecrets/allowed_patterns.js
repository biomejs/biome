/* should not generate diagnostics for allowed patterns */

// Stripe checkout session IDs - these are not secrets
const checkoutSession = "cs_test_a1XjM2B3cD4eF5gH6iJ7kL8m";
const liveCheckoutSession = "cs_live_a1XjM2B3cD4eF5gH6iJ7kL8m";
const publishableKey = "pk_test_a1B2c3D4e5F6g7H8i9J0kLmN";

/* should still detect actual secrets */

// AWS key should still be detected
const awsKey = "AKIA1234567890EXAMPLE";
