## sauna-monitor-backend

### setting diesel

create postgre user and password.

db : sauna
user : sauna
pass :sauna

set env variable of postgre sql
```sh
export DATABASE_URL=postgres://sauna:sauna@localhost/sauna
```

set up db with diesel.
```sh
diesel setup
```

```sh
diesel migration generate sensor_data
```

fix `./migrations/***sensor_data./down.sql` , `up.sql` 

```sh
diesel migration run
```
