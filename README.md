# mybackend

## when a track is played, the latest date and time the playback began should be recorded

### search last playback of track

GET /track_history/{track_id}

### update last playback of track

PUT /track_history/{track_id}

## The database can store when the album was last played.

### search last playback of album

GET /album_history/{album_id}

### update last playback of album

PUT /album_history/{album_id}

## users can search the artists or albums in the database

GET /artist/{artist_name}

GET /album/{album_name}

## users can add or remove the albums in the database to/from their personal collection

### search my favorite albums

GET /favorite_albums

### add my favorite albums

GET /favorite_albums/{album_id}

### remove my favorite albums

DELETE /favorite_albums/{album_id}

## users can search the albums or artists in the database, and add into their own collection

### search favorite artists

GET /favorite_artists

### add favorite artists

GET /favorite_artists/{album_id}

### remove favorite artists

DELETE /favorite_artists/{album_id}

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

POST /album/issue/{album_id}

### admin upload

#### create album

POST /album

## administrator can freely remove the albums in the database

### admin remove

DELETE album/{album_id}

## users can also upload new albums, but they are needed to be verified by the administrator

### user upload

TODO

## administrator can send messages to indicate whether a new album can be issued

TODO: add field message in album

## readers can make comments on the albums

POST comment/{album_id}

DELETE comment/{album_id}/{comment_id}

GET comment/{album_id}
