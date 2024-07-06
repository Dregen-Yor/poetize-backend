-- Please modify the database connection string in .env and config/config.toml first 
 -- Make sure the database exists, then run diesel migration run to restore the database, and run the following SQL in the database to add the default data. 
 -- After running, you can use the default username:zhangsan and password:123 to access /login. 
 -- For more diesel-cli functionality, please check /migration/README.md.
BEGIN;
INSERT INTO users (id, username, password) VALUES ('cdd0e080-5bb1-4442-b6f7-2ba60dbd0555', 'zhangsan', '$argon2id$v=19$m=19456,t=2,p=1$rcosL5pOPdA2c7i4ZuLA4Q$s0JGh78UzMmu1qZMpVUA3b8kWYLXcZhw7uBfwhYDJ4A');
COMMIT;
