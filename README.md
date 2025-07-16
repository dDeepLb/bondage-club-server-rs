## Using Docker with bondage club for development
 1. Install [Docker](https://docs.docker.com/get-docker/)
 2. Make sure you also install `docker-compose`, Docker Desktop comes with this tool, but Linux does not. If you already have Docker installed, make sure it's at least of version `18.06.0` or higher.

 ```sh
git clone git@github.com:dDeepLb/bondage-club-server-rs.git # clone repo
cd bondage-club-server-rs # cd into repo
cp .env.example .env # copy .env.example to .env
just fetch # download dependencies
just start # build and start containers
 ```
  `just start` command can be repeated to update the `bondage-club-server-rs` if you've made server changes.
<!-- 
Make the required changes to index.html in your Bondage-Club repository, and it will now be available at http://localhost/BondageClub/ -->

 * Mongo runs at localhost:27017, by default with the username and password `admin` and `password`, this can be changed in the .env file `MONGO_INITDB_ROOT_USERNAME` and `MONGO_INITDB_ROOT_PASSWORD`
 * Mongo-Express runs in a separate container at http://localhost:8081 and lets you access the database

### Convenience commands
You can list available commands by entering `just -l` into your terminal or find them in [`justfile`](./justfile).