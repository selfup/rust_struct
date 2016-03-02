CREATE TABLE previous_positions (
  id SERIAL PRIMARY KEY,
  position VARCHAR NOT NULL,
  dead BOOLEAN NOT NULL DEFAULT 'f'
)
