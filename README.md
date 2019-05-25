# Rocket, Diesel, MySQL, Rest API Tutorial

Build Restful CRUD API within our Database using Rocket and Diesel.

## Steps to Setup on your machine

**1. Clone the application**

```bash
git clone git@github.com:iamsimakov/rocket-diesel-rest-api-example.git
```
**3. Build and run the app using cargo**

```bash
cargo build --release && cd target/release/
sudo ROCKET_ENV=prod ./tasks-api
```

The app will start running at <http://localhost:80>.

Alternatively, you can run the app in development mode -

```bash
cargo run
```

## Steps to Run dev on your machine

**1. Clone the application**

```bash
git clone git@github.com:iamsimakov/rocket-diesel-rest-api-example.git
```

**2. Install docker, docker-compose**

https://docs.docker.com/install/linux/docker-ce/ubuntu/

**3. Run**

```bash
docker-compose up -d --build
```

## Explore Rest APIs

The app defines following CRUD APIs.

    GET /tasks
    
    POST /tasks
    
    PUT /tasks/{taskId}
    
    DELETE /tasks/{taskId}

You can test them using postman or any other rest client.

Based on git@github.com:sean3z/rocket-diesel-rest-api-example.git
