# mybackend

## healthcheck

GET /healthcheck

return ok

## auth

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

will return a token

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

will return a token

## user

### get user info (need token)

GET /user

### update user info (need token)

PUT /user

## artist

### search artists by name

GET /artist/{artist_name}

### create artist (need token)

POST /artist

```json
{
    "new_artist": {
        "name": "Khalil Fong"
    }
}
```

## album

### search albums by name

GET /album/{album_name}

### admin issue (need token)

POST /album/issue/{album_id}

#### create album (need token)

POST /album

```json
{
    "new_album": {
        "artist_id": 1,
        "name": "Wonderland"
    }
}
```

### admin remove (need token)

DELETE album/{album_id}

## audio

### admin upload & user upload track audio file (need token)

POST /audio/{track_id}

example:
```html
<html>
    <head><title>Upload Test</title></head>
    <body>
        <form target="/audio/1" method="post" enctype="multipart/form-data">
            <input type="file" multiple name="file"/>
            <button type="submit">Submit</button>
        </form>
    </body>
</html>
```

## favorite albums

### search my favorite albums (need token)

GET /favorite_albums

### add my favorite albums (need token)

GET /favorite_albums/{album_id}

### remove my favorite albums (need token)

DELETE /favorite_albums/{album_id}

## favorite artists

### search my favorite artists (need token)

GET /favorite_artists

### add my favorite artists (need token)

GET /favorite_artists/{artist_id}

### remove favorite artists (need token)

DELETE /favorite_artists/{artist_id}

## track history

### search last playback of track (need token)

GET /track_history/{track_id}

### update last playback of track (need token)

PUT /track_history/{track_id}

## album history

### search last playback of album (need token)

GET /album_history/{album_id}

### update last playback of album (need token)

PUT /album_history/{album_id}

## comment

### readers make comments on the albums (need token)

POST comment/{album_id}

```json
{
    "comment": {
        "body": "hehe"
    }
}
```

### readers delete a comment on the ablum (need token, author or admin)

DELETE comment/{comment_id}

### get all comments of an album

GET comment/{album_id}

---

## administrator can send messages to indicate whether a new album can be issued

TODO: add field message in album