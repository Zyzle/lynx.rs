# Lynx.rs

In Greek, Norse, and North American mythologies lynx were known as mysterious creatures and keepers of secrets. This one provides a simple API endpoint to allow you to fulfil the GitHub OAuth flow without exposing your client secret to a JS front-end.

## Deploying

The app will automatically link through to the GitHub API endpoint and will need to following environment variables to be set:

 * CLIENT_ID 
    The client ID given when you registered your GitHub app

 * CLIENT_SECRET
    The secret for the GitHub app

 * GITHUB_BASE
    The github access_token api url `https://github.com/login/oauth/access_token`

### Similar projects 

[prose/gatekeeper](https://github.com/prose/gatekeeper) project performs a similar function and is written in JS, although appears to have fallen out of active development.