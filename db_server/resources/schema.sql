CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username TEXT UNIQUE NOT NULL
);


CREATE TABLE conversations (
    conversation_id UUID PRIMARY KEY,
    user_id INT REFERENCES users(user_id) ON DELETE CASCADE,
    conversation_text TEXT NOT NULL
);
