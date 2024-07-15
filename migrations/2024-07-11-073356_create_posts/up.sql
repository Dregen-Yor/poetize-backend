-- Your SQL goes here

CREATE TABLE user (
  id SERIAL PRIMARY KEY,
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
  introduction VARCHAR(4096),
  user_type SMALLINT DEFAULT 2,
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  update_by VARCHAR(32),
  deleted SMALLINT DEFAULT 0
);


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

CREATE TABLE sort (
  id SERIAL PRIMARY KEY,
  sort_name VARCHAR(32) NOT NULL,
  sort_description VARCHAR(256) NOT NULL,
  sort_type SMALLINT DEFAULT 1,
  priority INTEGER
);


CREATE TABLE label (
  id SERIAL PRIMARY KEY,
  sort_id INTEGER,
  label_name VARCHAR(32) NOT NULL,
  label_description VARCHAR(256)
);

CREATE TABLE tree_hole (
  id SERIAL PRIMARY KEY,
  avatar VARCHAR(256),
  message VARCHAR(64) NOT NULL,
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


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


CREATE TABLE history_info (
  id SERIAL PRIMARY KEY,
  user_id INTEGER,
  ip VARCHAR(128) NOT NULL,
  nation VARCHAR(64),
  province VARCHAR(64),
  city VARCHAR(64),
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE sys_config (
  id SERIAL PRIMARY KEY,
  config_name VARCHAR(128) NOT NULL,
  config_key VARCHAR(64) NOT NULL,
  config_value VARCHAR(256),
  config_type CHAR(1) NOT NULL
);

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

CREATE TABLE im_chat_user_friend (
  id SERIAL PRIMARY KEY,
  user_id INTEGER,
  friend_id INTEGER,
  friend_status SMALLINT,
  remark VARCHAR(32),
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

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

CREATE TABLE im_chat_user_message (
  id BIGSERIAL PRIMARY KEY,
  from_id INTEGER,
  to_id INTEGER,
  content VARCHAR(1024) NOT NULL,
  message_status SMALLINT DEFAULT 0,
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE im_chat_user_group_message (
  id BIGSERIAL PRIMARY KEY,
  group_id INTEGER,
  from_id INTEGER,
  to_id INTEGER,
  content VARCHAR(1024) NOT NULL,
  create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
