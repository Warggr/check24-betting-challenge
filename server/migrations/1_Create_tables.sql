CREATE TABLE IF NOT EXISTS users (
  name VARCHAR(40) NOT NULL,
  points INTEGER NOT NULL,
  id INTEGER PRIMARY KEY NOT NULL -- in SQLite, autoincrement is automatic
);

CREATE TABLE IF NOT EXISTS communities (
  name VARCHAR(40) NOT NULL,
  id INTEGER PRIMARY KEY NOT NULL
);

CREATE TABLE IF NOT EXISTS users_x_communities (
  user_id INTEGER NOT NULL,
  community_id INTEGER NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id),
  FOREIGN KEY (community_id) REFERENCES communities(id),
  PRIMARY KEY(user_id, community_id)
);

CREATE TABLE IF NOT EXISTS games (
    team_home VARCHAR(20) NOT NULL,
    team_away VARCHAR(20) NOT NULL,
    starts_at TEXT,
    id INTEGER NOT NULL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS bets (
  user_id INTEGER NOT NULL,
  game_id INTEGER NOT NULL,
  bet_team_home INTEGER NOT NULL,
  bet_team_away INTEGER NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id),
  FOREIGN KEY (game_id) REFERENCES games(id),
  PRIMARY KEY (user_id, game_id)
);
