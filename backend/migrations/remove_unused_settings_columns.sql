-- Migration to remove unused settings columns
-- Run this SQL command in your PostgreSQL database

ALTER TABLE settings 
  DROP COLUMN IF EXISTS website_color,
  DROP COLUMN IF EXISTS gradient_color_1,
  DROP COLUMN IF EXISTS gradient_color_2,
  DROP COLUMN IF EXISTS gradient_color_3,
  DROP COLUMN IF EXISTS upload_box_transparency,
  DROP COLUMN IF EXISTS home_background,
  DROP COLUMN IF EXISTS ascii_color_mode;

-- Verify the changes
SELECT column_name, data_type 
FROM information_schema.columns 
WHERE table_name = 'settings' 
ORDER BY ordinal_position;
