# CHECK24 GenDev Betting Challenge

A server and web client for a betting game for the European Championship 2024.
This is a submission for the [Check24 GenDev Challenge](https://github.com/check24-scholarships/check24-betting-challenge).

A video can be found under [this link](https://drive.google.com/file/d/1_DNNCCU6EFadQWeGUO8004WV0Ts4Uavh/view?usp=sharing)

# Deployment

The back-end is written in [Rust](https://www.rust-lang.org/) and the front-end is a [Vue.js](https://vuejs.org/) app. Therefore, you need both [Rust's `cargo`](https://www.rust-lang.org/tools/install) and [Node.js' `npm`](https://nodejs.org/en/download) installed.

Build the front-end:
```
cd client
npm install
npm run build
```
Run the back-end:
```
cd server
cargo run
```

The app will then be available on `http://localhost:8000`.

# System architecture

The app consists of three layers:
- A database (currently Sqlite)
- A Rust back-end
- A web front-end

Fundamentally, everything is possible via directly manipulating the database. But some features have not yet been translated into server API endpoints, or are available as endpoints but have no UI elements in the client.

For more details, see [DEVELOPERS.md](./DEVELOPERS.md)

# Features

## Signup and login
Users can sign up with an username and then log in by providing the username (no password).

*Login is currently stored in a short-lived cookie and does not persist across page reloads.*

*The back-end does not usually check whether users are authorized to do what they are doing - it is trivial to provide a different user ID and bet for another player.*

## Communities
Users can create communities, and be part of multiple communities. They can see an overview with the top users, themselves, and the bottom users of a community on their home page.

*The front-end does not allow adding other users to communities.*

*Users can be part of any number of communities, not just 5.*

## Betting

Betting is possible as an API endpoint, but has not been integrated into the front-end. Betting is also currently possible at any time (even while the game is in progress).

## Real-time updates

Real-time updates was supposed to be implemented with [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events).
A proof-of-concept SSE endpoint is implemented on the server (see [realtime.rs](./server/src/realtime.rs) and the `/test/events` endpoint),
but it does currently not do anything useful.

## Features not implemented

- pinning friends
- community leaderboards

# Performance

Performance is a key aspect of the challenge.
The application needs to be able to handle up to 2 million users, with up to 10.000 users active at the same time observing the current standings.

## The front-end
The front-end is a single-page app (SPA), i.e. the actual front-end consists of only three documents (HTML, JS, and CSS code)
that are fetched only once when opening the page and that can also be cached efficiently (since they mostly don't change).
This reduces the load on the server.

## The API

When fetching lists of users, we never fetch the full list; instead, the client asks for a page with 10 users at a certain offset.
(The page size could be cranked up to 100 pretty easily).

The pages always have an offset that's a multiple of 10; it is e.g. not possible to ask for users 42 through 52.
This also improves cacheability, since all clients will send the same standardized requests (and not some subtly different requests).

## The server
The server is written in the [Rust](https://www.rust-lang.org/) language, which is a low-level, compiled language with no garbage collection.
This makes it very performant.

The server essentially does not hold any state on its own; all state is in the database.
This means we could scale the server horizontally and let the database handle the large number of connections.

## Real-time updates
Currently, no real-time updates are implemented.

However, in the long term, my plan was the following. Instead of updating the database whenever a goal is scored, the database would be updated only once at the end of the game.
The real-time updates would be done by an intermediary algorithm, which would query the server a few times to get the rankings before the game, listen to the "goal scored" events,
and exposes endpoints similar to those of the server, but representing the rankings "as though the game was finished with the current score".

Ideally, the intermediary algorithm (IA) would not keep the current rankings in memory, or even the complete list of users.
Instead, it would try to evaluate every query lazily (and possibly cache it); and in particular not expand / load into memory lists of users that are not necessary
(because their points are not close to any user we might look at).

An example lazy evaluation would be like this:
1. the client asks on which page of the rankings the current user with id $ID is.
1. the IA asks the server how many points the user with id $ID has.
1. the IA asks the server for a histogram of the number of users per points (that's pretty straightforward for the server to do with a `GROUP BY` query.
1. the IA sums all users that have more than 8 points more than the current user - these are definitely still in front in the rankings
1. the IA checks whether the current user bet on the correct (currently) winning team. If not, we add all users with more points than the current user to the total of users in front (the user made a wrong bet and has definitely not caught up on them).
1. the IA asks the server for more fine-grained statistics - the number of users within +8 (if the user was correct) or -8 (if wrong) points of the current user, grouped by who they bet would win the current game.
1. ...(I don't have time to write the full algorithm, I hope you see where this is going - we basically get coarse statistics on numbers of users far from the user, and refine progressively on users that might be either above or below the current user, until we get an exact ranking)
1. we could then keep these users around the current user into memory, because probably the client will ask to see the page with them and the 9 people around them soon.

The IA is a general concept and could be implemented in multiple way. It could be either a second server (or a second set of endpoints on the server), or it could be part of the client-side logic.
In fact, it is best if it has to track a small set of users (because then it only cares about a few places within the rankings, and does not have to ask the server for detailed info / lists of users at any other places);
therefore, implementing the IA in the client (maybe as a service worker?) would be ideal.
