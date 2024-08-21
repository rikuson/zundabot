# zundabot

## 環境

- MacBook Air (M1, 2020)
- macOS Monterey

## Gstreamerのインストール

[こちら](https://gstreamer.freedesktop.org/download/#macos)のページから「1.24.6 runtime installer」と「1.24.6 development installer」をダウンロード・インストール。  
`/Library/Frameworks/GStreamer.framework` の配下にファイルが作成されます。

## VOICEVOXのインストール

```shell
cd /usr/local
sudo mkdir lib
sudo chmod o+w lib
cd lib
curl -sSfLO https://github.com/VOICEVOX/voicevox_core/releases/download/0.15.4/download-osx-arm64
chmod +x download-osx-arm64
./download-osx-arm64
mv voicevox_core/* .
rm -r voicevox_core download-osx-arm64
```



```
"open_jtalk_dic_utf_8-1.11" -> /usr/local/lib/open_jtalk_dic_utf_8-1.11
```

