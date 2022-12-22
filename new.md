# new

用户查询：

查询{音乐家名称}=>{音乐家，专辑}

GET /artist/{artist_name}

output: ArtistResponse

```json
{
    "artists": [
        {
            "artist": {
                "id": 1,
                "name": "Khalil Fong"
            },
            "albums": [
                {
                    "id": 1,
                    "name": "Wonderland",
                    "last_playback": 239239423,
                    "artist_id": 1,
                    "issued": true
                }
            ]
        }
    ]
}
```

{专辑id}=>{专辑包含音乐名称}

GET /album/{album_id}

output: AlbumResponse

```json
{
    "albums": [
        {
            "album": {
                "id": 2,
                "name": "Dangerous World",
                "last_playback": 239239423,
                "artist_id": 1,
                "issued": true
            },
            "tracks": [
                {
                    "id": 2,
                    "name": "Mr. Weather",
                    "last_playback": 239239423,
                    "url": "fdsfsdf",
                    "artist_id": 1,
                    "album_id": 2,
                },
                {
                    "id": 3,
                    "name": "Peace",
                    "last_playback": 239239423,
                    "url": "dfsdf",
                    "artist_id": 1,
                    "album_id": 2,
                }
            ]
        }
    ]
}
```

播放功能{时间，音乐id}=>(修改数据)

GET /album/play/{album_id}

output: same as AlbumResponse

GET /track/play/{track_id}

output: TrackResponse

```json
{
    "tracks": [
        {
            "track": {
                "id": 2,
                "name": "Mr. Weather",
                "last_playback": 342343243,
                "url": "fdsfsdf",
                "artist_id": 1,
                "album_id": 2
            },
            "album_name": "Dangerous World",
            "artist_name": "Khalil Fong",
            "comments": [
                {
                    "id": 3,
                    "album_id": 2,
                    "author_id": 99, // user, not artist
                    "body": "I love Khalil Fong!",
                    "created_at": 2334324
                }
            ]
        }
    ]
}
```

收藏：{专辑id}=>{做出修改}

GET /favorite_albums/{album_id}

output: FavoriteAlbumResponse

```json
{
    "albums": {
        {
            "album": {
                "id": 2,
                "name": "Dangerous World",
                "last_playback": 239239423,
                "artist_id": 1,
                "issued": true
            },
            "artist_name": "Khalil Fong"
        }
    }
}
```

PUT /favorite_albums/{album_id}

GET /favorite_artists/{artist_id}

output: FavoriteArtistResponse

```json
{
    "artists": [
        {
            "artist": {
                ...
            },
            "albums": [
                {
                    ...
                }
            ]
        }
    ]
}
```

PUT /favorite_artists/{artist_id}

评论专辑：{专辑id,message}

GET /comment/{album_id}

output:

```json
{
    "comments": [
        {
            "id": "32",
            "album_id": 4543,
            "author_id": 45654765,
            "body": "fxxk",
            "created_at": 434564
        }
    ]
}
```

POST /comment/{album_id}

input:

```json
{
    "comment": {
        "body": "xfdfsdfdsfd"
    }
}
```

用户发布{艺术家名称，音乐名称，专辑名称，url}

input:

```json
{
    "new_album": {
        "artist_name": "Khalil Fong",
        "album_name": "15",
        "tracks": [
            {
                "name": "Bad",
                "url": "fdsfsdf"
            },
            {
                "name": "Read Bean",
                "url": "fdsfsdf"
            }
        ]
    }
}
```

output: album info, same as above

管理员查询：
查询{}=>{用户信息}

GET /user/all

查询{}=>{音乐家id,音乐家名称}

GET /artist/all

查询{}=>{专辑id,专辑名称，音乐家名称,专辑评论,最进播放时间}

GET /album/all

查询{}=>{音乐id,音乐name,专辑名称，音乐家名称，最近播放时间}

GET /track/all

查询发布请求{}=>{新发布艺术家名称，艺术家id,}（按钮：管理员自己审核自己）

GET /album/unissued
