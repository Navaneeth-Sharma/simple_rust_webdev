CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(64) NOT NULL UNIQUE,
  password VARCHAR(64) NOT NULL,
  token TEXT DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS tasks (
  id SERIAL PRIMARY KEY,
  priority VARCHAR(4) DEFAULT NULL,
  title VARCHAR(255) NOT NULL,
  description TEXT DEFAULT NULL,
  is_default BOOLEAN DEFAULT FALSE,
  CONSTRAINT fk_users FOREIGN KEY(user_id) REFERENCES users(id) 
);


INSERT INTO users (
  username,
  password
) VALUES ( 'user_a', '12345' );

INSERT INTO tasks (
  priority,
  title,
  description,
  is_default
) VALUES ('A', 'Bring Groceries', 'This is my exp', true),
('B', 'Read RUST', 'This is time to learn', true);

