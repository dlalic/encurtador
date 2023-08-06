CREATE TABLE c (
   id UUID PRIMARY KEY,
   url TEXT NOT NULL UNIQUE,
   shortened TEXT NOT NULL UNIQUE,
   created_at TIMESTAMP NOT NULL DEFAULT NOW(),
   updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('urls');