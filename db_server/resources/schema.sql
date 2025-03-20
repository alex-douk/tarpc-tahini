CREATE TABLE users ( user_id CHAR(36) PRIMARY KEY, username TEXT UNIQUE NOT NULL, targeted_ads_consent TINYINT);
CREATE TABLE conversations ( conversation_id CHAR(36) PRIMARY KEY, user_id CHAR(36) REFERENCES users(user_id) ON DELETE CASCADE, conversation_text TEXT NOT NULL, local_storage TINYINT, ads_consent TINYINT, image_gen_consent TINYINT, targeted_ads_consent TINYINT);
