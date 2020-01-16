# Readme

## Flows

## Glossary

It's implements simplified OAuth 2.0 flow ([example](https://itnext.io/an-oauth-2-0-introduction-for-beginners-6e386b19f7a9))

- Application — OAuth Client App
- User — the person who wants to be authenticated, to access protected information.
- Authmenow — Authorization server

### Authorization flow

Client side:

1. User wants to login. Open https://application/login
2. Application (redirects|opens a window) to https://authmenow/session?application_id&redirect_uri&state
3. Authmenow checks application request (application_id matches redirect_uri)
4. Authmenow shows login form
5. User inserts credentials
6. Authmenow checks credentials
7. Authmenow sends authorization_code to redirect_uri

Server side:

8. Application sends authorization_code, application_id and secret_key to Authmenow
9. Authmenow checks authorization_code (application_id matches secret_key, matches authorization_code)
10. Authmenow sends access_token back to Application

11. Application makes request using access_token to Authmenow to get info about session
12. Authmenow checks access_token
13. Authmenow returns info about session back to Application