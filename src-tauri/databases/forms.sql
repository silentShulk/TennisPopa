CREATE TABLE RegistrationFormItems (
    id INTEGER PRIMARY KEY,
    is_primary BOOL NOT NULL DEFAULT 0,
    form_url TEXT NOT NULL,
    form_id TEXT NOT NULL,
    question_ids TEXT NOT NULL
);

CREATE TABLE AvailabilityFormItems (
    id INTEGER PRIMARY KEY,
    form_url TEXT NOT NULL,
    form_id TEXT NOT NULL,
    question_ids TEXT NOT NULL
);