[Paper Wars]: https://github.com/foxfriends/paper-wars
[paper-wars-data]: https://github.com/foxfriends/paper-wars-data
[client]: https://github.com/foxfriends/paper-wars-client
[scryer-prolog]: https://github.com/mthom/scryer-prolog

# Paper Wars App Server

The server for [Paper Wars][], exposing a GraphQL API to all of the data required to view the
game, as well as the endpoints to interact with the it. The [client][] library is provided to
actually perform these requests.

See also:
*   The database is handled by the [paper-wars-data][] repository.
*   The game logic will be handled elsewhere also, eventually.

## Setup

1.  Set up the Database (see [paper-wars-data][]).
2.  Copy the `.env.sample` file to `.env` and fill it with the correct values. The database
    credentials should be as you set them when setting up the database.

    ```sh
    DATABASE_URL=postgres://paper-wars-server:<password>@localhost/paper-wars
    ```

## Engine

For now, there is an `/engine` directory. This may eventually be moved to its own repository.

The engine is written for [scryer-prolog][]. It will be used to process the scripts associated
with archetypes/maps/etc.
