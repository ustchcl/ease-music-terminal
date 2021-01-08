## 须知
```shell
apt install libasound2-dev
```

## v0.1
- 列表
- 播放

## v0.2
- 进度条
- 音量控制 `-` `+`
- 暂停和播放 `Space`


## v0.3
- [x] 下一首, 上一首
- [x] 自动下一首
- [x] 修复BUG

## v0.4
- [x] Login
- [x] dotenv login
- [x] 歌词显示

### 网易云API

#### 登陆
`/login/cellphone?phone={phone}&password={password}`


#### 获取用户详情
`/user/detail?uid=36005712`
和登陆返回一致, 除了cookie

#### 获取用户歌单
`/user/playlist?uid=36005712`

#### 获取歌单详情
`/playlist/detail?id=28251213`

### 获取音乐url
`/song/url?id=316486`

```json
{
    "data": [
        {
            "id": 316486,
            "url": "http://m8.music.126.net/20210102151538/bf80be75e337978e3dc1f37df1c98985/ymusic/obj/w5zDlMODwrDDiGjCn8Ky/3058340315/d15f/15c7/8b4b/6dc0489fd1c390045a4fa3fccbd6825f.flac",
            "br": 886294,
            "size": 24854798,
            "md5": "6dc0489fd1c390045a4fa3fccbd6825f",
            "code": 200,
            "expi": 1200,
            "type": "flac",
            "gain": 0,
            "fee": 8,
            "uf": null,
            "payed": 1,
            "flag": 4,
            "canExtend": false,
            "freeTrialInfo": null,
            "level": null,
            "encodeType": null,
            "freeTrialPrivilege": {
                "resConsumable": false,
                "userConsumable": false
            },
            "urlSource": 0
        }
    ],
    "code": 200
}

```