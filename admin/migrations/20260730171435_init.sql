CREATE TABLE Account (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE
);

CREATE TABLE Email (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    owner INTEGER NOT NULL,
    name TEXT NOT NULL UNIQUE,

    FOREIGN KEY(owner)
        REFERENCES Account(id)
);

CREATE TABLE Token (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    token TEXT NOT NULL UNIQUE,

    FOREIGN KEY(user_id)
        REFERENCES Account(id)
);

CREATE TABLE TokenGrant (
    token_id INTEGER NOT NULL,
    email_id INTEGER NOT NULL,

    is_read_only BOOLEAN NOT NULL,
    is_personal BOOLEAN NOT NULL,

    PRIMARY KEY (token_id, email_id),

    FOREIGN KEY (token_id) REFERENCES Token(id),
    FOREIGN KEY (email_id) REFERENCES Email(id)
)
