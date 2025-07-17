# Build the server with any changes and starts it up
run:
  docker-compose up --build

# Build the server with any changes and starts it up in detouched mode
start:
  docker-compose up -d --build

# Tear down and remove all containers but not the mongo database
stop:
  docker-compose stop

# Stops the server and starts it again rebuilding with any changes
restart:
  just stop && just start

# Tear down and remove all containers but not the mongo database
reset:
  docker-compose down

# Remove all the containers and the mongo database
reset-hard:
  docker-compose down --volumes

# View the logs from the bondage club server
logs-app:
  docker-compose logs app

# View the logs of Mongo
logs-db:
  docker-compose logs db
