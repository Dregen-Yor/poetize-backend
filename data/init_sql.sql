-- Please modify the database connection string in .env and config/config.toml first 
 -- Make sure the database exists, then run diesel migration run to restore the database, and run the following SQL in the database to add the default data. 
 -- After running, you can use the default username:zhangsan and password:123 to access /login. 
 -- For more diesel-cli functionality, please check /migration/README.md.
CREATE DATABASE poetize;

\c poetize;

DROP TABLE IF EXISTS user;

CREATE TABLE user (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  username VARCHAR(32) UNIQUE,
  password VARCHAR(128),
  phone_number VARCHAR(16),
  email VARCHAR(32),
  user_status SMALLINT DEFAULT 1,
  gender SMALLINT,
  open_id VARCHAR(128),
  avatar VARCHAR(256),
  admire VARCHAR(32),
  subscribe TEXT,
  introduction TEXT,
  user_type SMALLINT DEFAULT 2,
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  update_by VARCHAR(32),
  deleted SMALLINT DEFAULT 0
);

DROP TABLE IF EXISTS article;

CREATE TABLE article (
  id SERIAL PRIMARY KEY,
  user_id INTEGER,
  sort_id INTEGER,
  label_id INTEGER,
  article_cover VARCHAR(256),
  article_title VARCHAR(32) NOT NULL,
  article_content TEXT NOT NULL,
  video_url VARCHAR(1024),
  view_count INTEGER DEFAULT 0,
  like_count INTEGER DEFAULT 0,
  view_status SMALLINT DEFAULT 1,
  password VARCHAR(128),
  tips VARCHAR(128),
  recommend_status SMALLINT DEFAULT 0,
  comment_status SMALLINT DEFAULT 1,
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  update_by VARCHAR(32),
  deleted SMALLINT DEFAULT 0
);

DROP TABLE IF EXISTS comment;

CREATE TABLE comment (
  id SERIAL PRIMARY KEY,
  source INTEGER NOT NULL,
  type VARCHAR(32) NOT NULL,
  parent_comment_id INTEGER DEFAULT 0,
  user_id INTEGER,
  floor_comment_id INTEGER,
  parent_user_id INTEGER,
  like_count INTEGER DEFAULT 0,
  comment_content VARCHAR(1024) NOT NULL,
  comment_info VARCHAR(256),
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS sort;

CREATE TABLE sort (
  id SERIAL PRIMARY KEY,
  sort_name VARCHAR(32) NOT NULL,
  sort_description VARCHAR(256) NOT NULL,
  sort_type SMALLINT DEFAULT 1,
  priority INTEGER
);

DROP TABLE IF EXISTS label;

CREATE TABLE label (
  id SERIAL PRIMARY KEY,
  sort_id INTEGER,
  label_name VARCHAR(32) NOT NULL,
  label_description VARCHAR(256)
);

DROP TABLE IF EXISTS tree_hole;

CREATE TABLE tree_hole (
  id SERIAL PRIMARY KEY,
  avatar VARCHAR(256),
  message VARCHAR(64) NOT NULL,
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS wei_yan;

CREATE TABLE wei_yan (
  id SERIAL PRIMARY KEY,
  user_id INTEGER,
  like_count INTEGER DEFAULT 0,
  content VARCHAR(1024) NOT NULL,
  type VARCHAR(32) NOT NULL,
  source INTEGER,
  is_public SMALLINT DEFAULT 0,
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS web_info;

CREATE TABLE web_info (
  id SERIAL PRIMARY KEY,
  web_name VARCHAR(16) NOT NULL,
  web_title VARCHAR(512) NOT NULL,
  notices VARCHAR(512),
  footer VARCHAR(256) NOT NULL,
  background_image VARCHAR(256),
  avatar VARCHAR(256) NOT NULL,
  random_avatar TEXT,
  random_name VARCHAR(4096),
  random_cover TEXT,
  waifu_json TEXT,
  status SMALLINT DEFAULT 1
);

DROP TABLE IF EXISTS resource_path;

CREATE TABLE resource_path (
  id SERIAL PRIMARY KEY,
  title VARCHAR(64) NOT NULL,
  classify VARCHAR(32),
  cover VARCHAR(256),
  url VARCHAR(256),
  introduction VARCHAR(1024),
  type VARCHAR(32) NOT NULL,
  status SMALLINT DEFAULT 1,
  remark TEXT,
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS resource;

CREATE TABLE resource (
  id SERIAL PRIMARY KEY,
  user_id INTEGER,
  type VARCHAR(32) NOT NULL,
  path VARCHAR(256) NOT NULL,
  size INTEGER,
  original_name VARCHAR(512),
  mime_type VARCHAR(256),
  status SMALLINT DEFAULT 1,
  store_type VARCHAR(16),
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (path)
);

DROP TABLE IF EXISTS history_info;

CREATE TABLE history_info (
  id SERIAL PRIMARY KEY,
  user_id INTEGER,
  ip VARCHAR(128) NOT NULL,
  nation VARCHAR(64),
  province VARCHAR(64),
  city VARCHAR(64),
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS sys_config;

CREATE TABLE sys_config (
  id SERIAL PRIMARY KEY,
  config_name VARCHAR(128) NOT NULL,
  config_key VARCHAR(64) NOT NULL,
  config_value VARCHAR(256),
  config_type CHAR(1) NOT NULL
);

DROP TABLE IF EXISTS family;

CREATE TABLE family (
  id SERIAL PRIMARY KEY,
  user_id INTEGER,
  bg_cover VARCHAR(256) NOT NULL,
  man_cover VARCHAR(256) NOT NULL,
  woman_cover VARCHAR(256) NOT NULL,
  man_name VARCHAR(32) NOT NULL,
  woman_name VARCHAR(32) NOT NULL,
  timing VARCHAR(32) NOT NULL,
  countdown_title VARCHAR(32),
  countdown_time VARCHAR(32),
  status SMALLINT DEFAULT 1,
  family_info VARCHAR(1024),
  like_count INTEGER DEFAULT 0,
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS im_chat_user_friend;

CREATE TABLE im_chat_user_friend (
  id SERIAL PRIMARY KEY,
  user_id INTEGER,
  friend_id INTEGER,
  friend_status SMALLINT,
  remark VARCHAR(32),
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS im_chat_group;

CREATE TABLE im_chat_group (
  id SERIAL PRIMARY KEY,
  group_name VARCHAR(32) NOT NULL,
  master_user_id INTEGER,
  avatar VARCHAR(256),
  introduction VARCHAR(128),
  notice VARCHAR(1024),
  in_type SMALLINT DEFAULT 1,
  group_type SMALLINT DEFAULT 1,
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS im_chat_group_user;

CREATE TABLE im_chat_group_user (
  id SERIAL PRIMARY KEY,
  group_id INTEGER,
  user_id INTEGER,
  verify_user_id INTEGER,
  remark VARCHAR(1024),
  admin_flag SMALLINT DEFAULT 0,
  user_status SMALLINT,
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS im_chat_user_message;

CREATE TABLE im_chat_user_message (
  id BIGSERIAL PRIMARY KEY,
  from_id INTEGER,
  to_id INTEGER,
  content VARCHAR(1024) NOT NULL,
  message_status SMALLINT DEFAULT 0,
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS im_chat_user_group_message;

CREATE TABLE im_chat_user_group_message (
  id BIGSERIAL PRIMARY KEY,
  group_id INTEGER,
  from_id INTEGER,
  to_id INTEGER,
  content VARCHAR(1024) NOT NULL,
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Insert data
INSERT INTO "user" (id, username, password, phone_number, email, user_status, gender, open_id, admire, subscribe, avatar, introduction, user_type, update_by, deleted) 
VALUES (1, 'Sara', '47bce5c74f589f4867dbd57e9ca9f808', '', '', 1, 1, '', '', '', '', '', 0, 'Sara', 0);

INSERT INTO web_info (id, web_name, web_title, notices, footer, background_image, avatar, random_avatar, random_name, random_cover, waifu_json, status) 
VALUES (1, 'Sara', 'POETIZE', '[]', '云想衣裳花想容， 春风拂槛露华浓。', '', '', '[]', '[]', '[]', '{}', 1);

INSERT INTO family (id, user_id, bg_cover, man_cover, woman_cover, man_name, woman_name, timing, countdown_title, countdown_time, status, family_info, like_count, create_time, update_time) 
VALUES (1, 1, '背景封面', '男生头像', '女生头像', 'Sara', 'Abby', '2000-01-01 00:00:00', '春节倒计时', '2025-01-29 00:00:00', 1, '', 0, '2000-01-01 00:00:00', '2000-01-01 00:00:00');

INSERT INTO im_chat_group (id, group_name, master_user_id, introduction, notice, in_type) 
VALUES (-1, '公共聊天室', 1, '公共聊天室', '欢迎光临！', 0);

INSERT INTO im_chat_group_user (id, group_id, user_id, admin_flag, user_status) 
VALUES (1, -1, 1, 1, 1);

INSERT INTO sys_config (id, config_name, config_key, config_value, config_type) 
VALUES 
(1, 'QQ邮箱号', 'spring.mail.username', '', '1'),
(2, 'QQ邮箱授权码', 'spring.mail.password', '', '1'),
(3, '邮箱验证码模板', 'user.code.format', '【POETIZE】%s为本次验证的验证码，请在5分钟内完成验证。为保证账号安全，请勿泄漏此验证码。', '1'),
(4, '邮箱订阅模板', 'user.subscribe.format', '【POETIZE】您订阅的专栏【%s】新增一篇文章：%s。', '1'),
(5, '默认存储位置', 'default.storage', '/home/POETIZE/images/', '1'),
(6, '是否开启注册', 'user.register', '1', '1'),
(7, '默认发件人', 'spring.mail.sender', 'POETIZE', '1'),
(8, '聊天端口', 'ws.port', '8888', '1'),
(9, '聊天服务器地址', 'ws.url', 'localhost', '1'),
(10, '默认存储位置类型', 'default.storage.type', 'local', '1');
