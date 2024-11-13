CREATE TABLE IF NOT EXISTS user_right (
      username VARCHAR(255),
      schedule_id BIGSERIAL REFERENCES schedule (id),
      write BOOLEAN
  )
