-- Rename the old status column if it exists (optional)
ALTER TABLE transactions
DROP COLUMN IF EXISTS status;

-- Add new columns
ALTER TABLE transactions
ADD COLUMN IF NOT EXISTS qr_status TEXT NOT NULL DEFAULT 'PENDING',
ADD COLUMN IF NOT EXISTS confirm_status TEXT NOT NULL DEFAULT 'PENDING',
ADD COLUMN IF NOT EXISTS user_id TEXT NULL;
