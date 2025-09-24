CREATE EXTENSION "uuid-ossp";

CREATE TABLE rr_user(
  id BIGSERIAL PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  email TEXT NOT NULL,
  password_hash TEXT NOT NULL,
  created_at timestamp with time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp with time zone NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX rr_user_email_unique ON rr_user (email);

CREATE TABLE activity(
  id BIGSERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  started_at timestamp with time zone NOT NULL,
  ended_at timestamp with time zone NOT NULL,
  created_at timestamp with time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp with time zone NOT NULL DEFAULT NOW()
);

CREATE TABLE session(
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id BIGINT references rr_user NOT NULL,
  created_at timestamp with time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp with time zone NOT NULL DEFAULT NOW()
);
