# mybackend

## users can search the artists or albums in the database

GET /artist/search/{artist_name}

GET /album/search/{album_name}

## users can add or remove the albums in the database to/from their personal collection

### search my favorite albums

GET /favorite/search

### add my favorite albums

GET /favorite/add/{album_id}

### remove my favorite albums

GET /favorite/remove/{album_id}

## personal users can register their unique accounts in the database

### register

POST /auth/register

```json
{
    "user": {
        "username": "admin",
        "email": "garen-wang@qq.com",
        "password": "admin"
    }
}
```

### login

POST /auth/login

```json
{
    "user": {
        "email": "garen-wang@qq.com",
        "password": "admin"
    }
}
```

## administrator can freely upload/issue new albums to the database

### admin issue


TODO: admin upload

## administrator can freely remove the albums in the database

### admin remove

## users can search the albums or artists in the database, and add into their own collection

### TODO: favorite artists

## users can also upload new albums, but they are needed to be verified by the administrator

### TODO: user upload

## administrator can send messages to indicate whether a new album can be issued

GET
TODO: field of album message

## readers can make comments on the albums

POST