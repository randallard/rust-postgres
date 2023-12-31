# rust-postgres
## Tech stack
- sqlx (cargo install sqlx-cli)
- cargo watch (cargo install cargo-watch)
- axum

# devContainers
- The .devcontainer folder creates a uniformed environment that allows developers to have consistency while coding. All tools, plugins, dependecies can be added to the appropriate file (Dockerfile,docker-compose, devcontainer.json) and as a result all of our environments will be the same. We can use any IDE that supports devcontainer, but VS Code has been tested.
- To connect to Postgres, Open the terminal `Ctrl + \`` and click ports. You should automatically see a green mark next to our two ports for this application, 3000 (application) 5432 (database).

# Build locally
- If devcontainers doesn't fit your flow, attached is a docker-compose that will spin up postgres database and and pgadmin for administration. 

# Developing 
**cargo watch lets us edit code and compile in real time, finding mistakes faster as a result**
- `cargo watch -q -c -w src/ -x run`


# Database migrations
- `cargo install sqlx-cli`
- `sqlx migrate add -r init`
