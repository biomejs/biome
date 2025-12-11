/* should not generate diagnostics - env vars declared in legacy pipeline */

// Legacy pipeline env vars should still work
const pipelineVar = process.env.PIPELINE_VAR;
const pipelineSecret = process.env.PIPELINE_SECRET;

// Global env vars
const globalVar = process.env.GLOBAL_VAR;
