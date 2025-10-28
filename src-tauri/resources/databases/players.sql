PRAGMA foreign_keys = ON;

CREATE TABLE PlayerGroup (
    id INTEGER PRIMARY KEY,
    category INTEGER NOT NULL
);

CREATE TABLE Player (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE ON CONFLICT REPLACE,
    phone_number TEXT NOT NULL,
    category INTEGER NOT NULL,
    date_of_creation TEXT NOT NULL,
    availability INTEGER NOT NULL DEFAULT 7,
    size INTEGER NOT NULL,

    id_group INTEGER,
    FOREIGN KEY (id_group) REFERENCES PlayerGroup(id) ON DELETE SET NULL
);

CREATE TABLE ScheduledMatch(
    player_1 INTEGER,
    player_2 INTEGER,
    scheduled_time INTEGER,
    court INTEGER,

    PRIMARY KEY (player_1, player_2),
    FOREIGN KEY (player_1) REFERENCES Player(id),
    FOREIGN KEY (player_2) REFERENCES Player(id),

    CHECK (player_1 < player_2),
    CHECK (player_1 <> player_2)
);

CREATE TABLE PlayerMatch (
    player_1 INTEGER,
    player_2 INTEGER,
    set_1_win INTEGER,
    set_1_lose INTEGER,
    set_2_win INTEGER,
    set_2_lose INTEGER,
    tie_win INTEGER,
    tie_lose INTEGER,
    

    PRIMARY KEY (player_1, player_2),
    FOREIGN KEY (player_1) REFERENCES Player(id),
    FOREIGN KEY (player_2) REFERENCES Player(id),

    CHECK (player_1 < player_2),
    CHECK (player_1 <> player_2)
);