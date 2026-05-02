# tiny-sns Mermaid図一覧

この文書は、ルートに分散していた Mermaid の `.mmd` ファイルを Markdown に統合したものです。
共有して使う図の正本はこのファイルとし、他の文書では必要に応じてこのファイルを参照します。

図だけを単独で開くよりも、以下の点が分かりやすくなるように整理しています。

- その図が何を説明しているか
- どこを見れば概要を掴めるか
- 他の設計資料とどうつながるか

## 図の使い分け

| 図 | 用途 | まず見るポイント |
| --- | --- | --- |
| 機能スコープ図 | tiny-sns のMVP範囲を俯瞰する | どの機能群があるか |
| 画面遷移図 | 主要画面の導線を把握する | 未ログインからホームまでの流れ |
| コアER図 | SNS中心機能のデータ関係を掴む | `users`、`posts`、`follows` の関係 |
| DBMLギャップ補完図 | 元のDBMLに何を補ったかを確認する | 追加したテーブルとカラム |

<a id="scope-diagram"></a>

## 1. 機能スコープ図

この図は、tiny-sns のMVPで扱う機能領域を大づかみに示しています。

読み方のポイントは次の通りです。

- 中央の `tiny-sns MVP` から、機能ドメインごとに枝分かれしています
- 1段目は機能カテゴリ、2段目は具体的な機能です
- 実装順を考えるときは、`認証・アカウント` と `投稿` と `タイムライン` を先に見ると全体像を掴みやすいです

```mermaid
flowchart TB
    service[tiny-sns MVP] --> account[認証・アカウント]
    service --> profile[プロフィール]
    service --> post[投稿]
    service --> timeline[タイムライン]
    service --> discovery[検索・発見]
    service --> social[交流]
    service --> notification[通知]
    service --> safety[安全管理]

    account --> signup[新規登録]
    account --> login[ログイン/ログアウト]
    account --> session[セッション管理]
    account --> verify[メール確認]
    account --> recovery[パスワード再設定]
    account --> privacy[公開設定]

    profile --> profile_view[プロフィール表示]
    profile --> profile_edit[プロフィール編集]

    post --> create_post[投稿作成]
    post --> edit_post[投稿編集]
    post --> delete_post[投稿削除]
    post --> reply_quote[返信/引用]
    post --> media_attach[メディア添付]
    post --> visibility[公開範囲設定]

    timeline --> home_tl[ホームタイムライン]
    timeline --> user_tl[ユーザー投稿一覧]
    timeline --> thread_view[スレッド表示]
    timeline --> bookmark_tl[ブックマーク一覧]

    discovery --> search[ユーザー/投稿/タグ検索]
    discovery --> hashtag_tl[ハッシュタグ投稿一覧]

    social --> follow[フォロー/解除]
    social --> follow_request[フォロー申請承認]
    social --> like[いいね]
    social --> repost[リポスト]
    social --> bookmark[ブックマーク]
    social --> mention[メンション]

    notification --> inbox[通知一覧]
    notification --> read_state[既読/未読]

    safety --> mute[ミュート]
    safety --> block[ブロック]
    safety --> report[通報]
```

補足:

- `検索・発見` は SNS としての最低限の導線です。タイムラインだけだと新規発見が弱くなるため、MVPでも分けて扱っています
- `安全管理` は後回しにされがちですが、ブロックと通報は最初から仕様に入れておく方が後戻りが少ないです

<a id="screen-flow-diagram"></a>

## 2. 画面遷移図

この図は、ユーザーがどの画面を経由して利用するかを示す高レベルな遷移図です。

読み方のポイントは次の通りです。

- 左側が未ログイン状態、中央以降がログイン後の主要導線です
- `ホームタイムライン` が中心ハブになっています
- `投稿詳細` と `プロフィール` から通報へ進めるようにしてあり、安全機能が画面上どこから使えるかも把握できます

```mermaid
flowchart LR
    guest[未ログイン] --> signup[新規登録]
    guest --> login[ログイン]
    guest --> reset_request[パスワード再設定申請]
    signup --> verify_wait[メール確認待ち]
    verify_wait --> login
    reset_request --> reset_done[再設定完了]
    login --> home[ホームタイムライン]

    home --> composer[投稿作成/編集]
    home --> post_detail[投稿詳細]
    home --> profile[プロフィール]
    home --> notifications[通知]
    home --> bookmarks[ブックマーク]
    home --> search[検索]
    home --> settings[公開/セキュリティ設定]

    post_detail --> composer
    post_detail --> report[通報]
    profile --> edit_profile[プロフィール編集]
    profile --> follow_list[フォロー/フォロワー一覧]
    profile --> report

    search --> hashtag[ハッシュタグ投稿一覧]
    settings --> requests[フォロー申請一覧]
    settings --> mute_block[ミュート/ブロック管理]
```

補足:

- これは全画面を網羅する詳細遷移図ではなく、MVPの主要導線に絞った図です
- 実装時はこの図を起点に、画面ごとの状態や API を別途細分化するのが扱いやすいです

<a id="core-er-diagram"></a>

## 3. コアER図

この図は、SNS の中心機能に関わる主要テーブルの関係を示しています。

読み方のポイントは次の通りです。

- まず `users` を起点に `posts`、`follows`、`notifications` を追うと主要ユースケースが見えます
- `post_media`、`post_hashtags`、`mentions` は、中核テーブルを補助する中間テーブルです
- `user_profiles.pinned_post_id` により、プロフィール先頭に表示する固定ツイートも表現できます
- 認証用テーブルや通報テーブルはこの図からは省き、コアSNS動作に集中しています

```mermaid
erDiagram
    USERS {
        uuid id PK
        varchar handle
        varchar display_name
        varchar email
        varchar status
        boolean is_private
    }
    USER_PROFILES {
        uuid user_id PK
        text bio
        varchar location
        varchar website
        uuid avatar_media_id FK
        uuid header_media_id FK
        uuid pinned_post_id FK
    }
    POSTS {
        uuid id PK
        uuid author_id FK
        text text
        uuid reply_to_post_id FK
        uuid quote_post_id FK
        varchar visibility
        timestamptz deleted_at
    }
    MEDIA {
        uuid id PK
        uuid owner_id FK
        varchar media_type
        varchar url
    }
    POST_MEDIA {
        uuid post_id FK
        uuid media_id FK
        int position
    }
    FOLLOWS {
        uuid follower_id FK
        uuid followee_id FK
        varchar status
    }
    LIKES {
        uuid user_id FK
        uuid post_id FK
    }
    REPOSTS {
        uuid user_id FK
        uuid post_id FK
    }
    BOOKMARKS {
        uuid user_id FK
        uuid post_id FK
    }
    HASHTAGS {
        uuid id PK
        varchar tag
    }
    POST_HASHTAGS {
        uuid post_id FK
        uuid hashtag_id FK
    }
    MENTIONS {
        uuid post_id FK
        uuid mentioned_user_id FK
    }
    NOTIFICATIONS {
        uuid id PK
        uuid recipient_id FK
        uuid actor_id FK
        uuid post_id FK
        varchar notification_type
        timestamptz read_at
    }
    BLOCKS {
        uuid blocker_id FK
        uuid blocked_id FK
    }
    MUTES {
        uuid muter_id FK
        uuid muted_id FK
    }

    USERS ||--|| USER_PROFILES : has
    USER_PROFILES o|--o| POSTS : pins
    USERS ||--o{ POSTS : authors
    USERS ||--o{ MEDIA : owns
    POSTS ||--o{ POST_MEDIA : contains
    MEDIA ||--o{ POST_MEDIA : attached
    USERS ||--o{ FOLLOWS : follower_side
    USERS ||--o{ FOLLOWS : followee_side
    USERS ||--o{ LIKES : likes
    POSTS ||--o{ LIKES : liked_by
    USERS ||--o{ REPOSTS : reposts
    POSTS ||--o{ REPOSTS : reposted_by
    USERS ||--o{ BOOKMARKS : bookmarks
    POSTS ||--o{ BOOKMARKS : bookmarked_by
    POSTS ||--o{ POST_HASHTAGS : tagged
    HASHTAGS ||--o{ POST_HASHTAGS : used_in
    POSTS ||--o{ MENTIONS : has_mentions
    USERS ||--o{ MENTIONS : mentioned_user
    USERS ||--o{ NOTIFICATIONS : recipient
    USERS o|--o{ NOTIFICATIONS : actor
    POSTS o|--o{ NOTIFICATIONS : target_post
    USERS ||--o{ BLOCKS : blocker_side
    USERS ||--o{ MUTES : muter_side
```

補足:

- この図は「SNSとしての中心機能」を見るための簡略版です
- 認証や通報も含めた拡張版は [docs/specification.md](/Users/ino/Dev/github/tiny-sns/docs/specification.md) で確認できます

<a id="dbml-gap-diagram"></a>

## 4. DBMLギャップ補完図

この図は、元の DBML に対して今回補った不足点を一覧で示しています。

読み方のポイントは次の通りです。

- 左が不足していた論点、右が追加した対策です
- 仕様変更の理由を短時間で確認したい時に最も見やすい図です
- DB 設計レビューの入口として使う想定です

```mermaid
flowchart TD
    gap1[非公開アカウント設定がない] --> fix1[users.is_private を追加]
    gap2[固定ツイートの保持先がない] --> fix2[user_profiles.pinned_post_id を追加]
    gap3[ログインセッションが保存できない] --> fix3[auth_sessions を追加]
    gap4[メール認証が保存できない] --> fix4[email_verification_tokens を追加]
    gap5[パスワード再設定が保存できない] --> fix5[password_reset_tokens を追加]
    gap6[通報の受付先がない] --> fix6[reports を追加]
    gap7[タイムライン向け索引が弱い] --> fix7[posts/follows/notifications に索引追加]
    gap8[メディア順序と状態が弱い] --> fix8[post_media 一意制約と media.processing_status を追加]
```

補足:

- 機能を増やしたというより、MVPとして運用可能にするための足りない基礎を埋めた図です
- 詳細理由は [docs/dbml_gap_analysis.md](/Users/ino/Dev/github/tiny-sns/docs/dbml_gap_analysis.md) に文章でまとめています

## 関連ドキュメント

- 全体仕様: [docs/specification.md](/Users/ino/Dev/github/tiny-sns/docs/specification.md)
- DBML不足項目の説明: [docs/dbml_gap_analysis.md](/Users/ino/Dev/github/tiny-sns/docs/dbml_gap_analysis.md)
- 最新スキーマ: [docs/sns.dbml](/Users/ino/Dev/github/tiny-sns/docs/sns.dbml)

今後さらに図を増やす場合も、この Markdown に同じ形式で追加していくと散らばりにくくなります。