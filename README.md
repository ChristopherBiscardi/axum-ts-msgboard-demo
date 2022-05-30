## TODO:

- [x] Axum API
  - [x] GET /messages
  - [x] POST /message
- [x] Axum Tests
- [ ] README
  - [ ] how to run the application
  - [ ] how to run the tests

### Constraints

- use axum
- use typescript
- tests for api routes
- no ui tests required
- in-memory storage (no db)

## Random thoughts on future work

- Messages should live in database some day
- Messages should get ids if we're going to enable functionality like replies. Maybe ksuid/ulid for some bonus inherent ordering.
- Form submission does not yield feedback when submitting
- polling every second is kinda meh as a solution for live updates. If that was a feature then maybe websockets or SSE
- lots of missing functionality: users, avatars, more than one thread on the site, ability to start new threads, reply to specific posts, etc.

---

called development at 2.5h with a working frontend and backend.
