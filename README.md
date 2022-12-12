## sauna-monitor-backend

### setting diesel

create postgre user and password.

db : sauna
user :sauna
pass :sauna

```sh
CREATE USER sauna WITH PASSWORD 'sauna';
CREATE DATABASE sauna;
```

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

```up.sql
# up.sql

CREATE TABLE sensor_data(
	id SERIAL PRIMARY KEY,
    time_stamp time,
    data0 real,
    data1 real,
    data2 real
);

```

```down.sql
# down.sql

DROP TABLE sensor_data;

```

```sh
diesel migration run
```
