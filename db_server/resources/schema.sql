-- |user_id | username | targeted_ads_consent|
CREATE TABLE users ( user_id CHAR(36) PRIMARY KEY, username TEXT UNIQUE NOT NULL, targeted_ads_consent TINYINT);
--  0           1          2         3     4         5               6             7                   8 
-- |message_id| conv_id | user_id | role | content | local_storage | ads_consent | image_gen_consent | targeted_ads_consent |
CREATE TABLE conversations (message_id SERIAL PRIMARY KEY, conversation_id CHAR(36), user_id CHAR(36) REFERENCES users(user_id) ON DELETE CASCADE, role VARCHAR(10), content TEXT NOT NULL,  local_storage TINYINT, ads_consent TINYINT, image_gen_consent TINYINT, targeted_ads_consent TINYINT);

INSERT INTO users VALUES ("84a2f6aa-b658-4f13-98ec-0b9f14318808", "anonymous", false);
