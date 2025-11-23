# Settings Persistence Fix Summary

## Problem
Settings were not persisting after page refresh despite correct values in the database. The root cause was a **snake_case vs camelCase mismatch** between backend and frontend.

### Root Causes Identified
1. **Database/Backend**: Used snake_case field names (`logo_path`, `navbar_title`, `max_upload_size`)
2. **Frontend**: Expected camelCase field names (`logo`, `navbarTitle`, `maxUploadSize`)
3. **No transformation layer**: Backend returned raw snake_case JSON, frontend couldn't map it correctly
4. **Unused fields**: Database contained deprecated fields that were cluttering the schema

## Changes Made

### Backend Changes

#### 1. `backend/src/models.rs` - Settings Struct
**Added serde rename attributes** to transform snake_case → camelCase:
```rust
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Settings {
    pub id: i32,
    pub theme: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "logo")]
    pub logo_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "backgroundImage")]
    pub background_path: Option<String>,
    #[serde(rename = "navbarTitle")]
    pub navbar_title: String,
    #[serde(rename = "maxUploadSize")]
    pub max_upload_size: i64,
    #[serde(rename = "blurIntensity")]
    pub blur_intensity: i32,
    #[serde(rename = "maxValidity")]
    pub max_validity: String,
    #[serde(rename = "allowRegistration")]
    pub allow_registration: bool,
    #[serde(rename = "expirationAction")]
    pub expiration_action: String,
}
```

**Removed deprecated fields**:
- ❌ `website_color`
- ❌ `gradient_color_1`, `gradient_color_2`, `gradient_color_3`
- ❌ `upload_box_transparency`
- ❌ `home_background`
- ❌ `ascii_color_mode`

#### 2. `backend/src/handlers/settings.rs` - Get Settings
**Updated SELECT query** to exclude removed fields:
```sql
SELECT id, theme, logo_path, background_path, navbar_title, max_upload_size,
       blur_intensity, max_validity, allow_registration, expiration_action
FROM settings ORDER BY id LIMIT 1
```

#### 3. `backend/src/handlers/admin.rs` - Update Settings
**Updated INSERT query**:
```sql
INSERT INTO settings (theme, logo_path, background_path, navbar_title, max_upload_size, 
                      blur_intensity, max_validity, allow_registration, expiration_action)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id
```

**Updated UPDATE query**:
```sql
UPDATE settings SET theme = $1, logo_path = $2, background_path = $3, navbar_title = $4, 
                    max_upload_size = $5, blur_intensity = $6, max_validity = $7, 
                    allow_registration = $8, expiration_action = $9, updated_at = CURRENT_TIMESTAMP
WHERE id = $10
```

### Frontend Changes

#### `frontend/pingo/src/pages/account/components/AdminSettings.vue`

**Simplified settings structure**:
```typescript
const settings = ref({
  logo: '',
  backgroundImage: '',
  navbarTitle: 'PinGO',
  maxUploadSize: 104857600,
  blurIntensity: 0,
  maxValidity: '7days',
  allowRegistration: true,
  expirationAction: 'unavailable'
})
```

**Fixed loadSettings** - now expects camelCase from backend:
```typescript
const loadSettings = async () => {
  const currentSettings = await getSettings()
  
  // Backend now returns camelCase due to serde rename
  settings.value = {
    logo: currentSettings.logo ?? '',
    backgroundImage: currentSettings.backgroundImage ?? '',
    navbarTitle: currentSettings.navbarTitle ?? 'PinGO',
    maxUploadSize: currentSettings.maxUploadSize ?? 104857600,
    blurIntensity: currentSettings.blurIntensity ?? 0,
    maxValidity: currentSettings.maxValidity ?? '7days',
    allowRegistration: currentSettings.allowRegistration ?? true,
    expirationAction: currentSettings.expirationAction ?? 'unavailable'
  }
}
```

**Removed**:
- Gradient color fields and dialog
- ColorPicker component (unused)
- Gradient-related functions

### Database Migration

Created `backend/migrations/remove_unused_settings_columns.sql`:
```sql
ALTER TABLE settings 
  DROP COLUMN IF EXISTS website_color,
  DROP COLUMN IF EXISTS gradient_color_1,
  DROP COLUMN IF EXISTS gradient_color_2,
  DROP COLUMN IF EXISTS gradient_color_3,
  DROP COLUMN IF EXISTS upload_box_transparency,
  DROP COLUMN IF EXISTS home_background,
  DROP COLUMN IF EXISTS ascii_color_mode;
```

## How to Apply

### 1. Run Database Migration
```bash
# Connect to PostgreSQL
psql -U your_user -d pingo_db

# Run the migration
\i backend/migrations/remove_unused_settings_columns.sql
```

### 2. Rebuild Backend
```bash
cd backend
cargo build --release
```

### 3. Restart Application
```bash
# Stop current backend
pkill pingo-share-backend

# Start new backend
./target/release/pingo-share-backend

# Frontend should hot-reload automatically
```

## Testing Checklist

- [ ] Settings load correctly on page load
- [ ] Logo upload works and persists
- [ ] Background image upload works and persists
- [ ] Navbar title changes persist
- [ ] Max upload size changes persist
- [ ] Blur intensity slider persists
- [ ] Max validity dropdown persists
- [ ] Allow registration toggle persists
- [ ] Expiration action dropdown persists
- [ ] Settings survive page refresh
- [ ] Browser console shows no errors
- [ ] Database values match UI after save

## Expected API Response Format

After these changes, the `/api/settings` endpoint returns:
```json
{
  "id": 1,
  "theme": "light",
  "logo": "/avatars/logo.png",
  "backgroundImage": "/backgrounds/bg.jpg",
  "navbarTitle": "PinGO",
  "maxUploadSize": 104857600,
  "blurIntensity": 5,
  "maxValidity": "7days",
  "allowRegistration": true,
  "expirationAction": "unavailable"
}
```

✅ **All field names are now in camelCase, matching frontend expectations!**

## Technical Details

### Serde Rename Behavior
- Database column: `logo_path` (snake_case)
- Rust struct field: `logo_path` (snake_case)
- JSON output: `"logo"` (camelCase via `#[serde(rename = "logo")]`)

This transformation happens automatically during serialization, so:
- No code changes needed in handlers
- Database schema remains snake_case (SQL convention)
- Frontend receives camelCase (JavaScript convention)

### Nullish Coalescing
Using `??` instead of `||` ensures:
- `false` values don't fallback to defaults
- `0` values don't fallback to defaults
- Empty strings `""` don't fallback to defaults
- Only `null` and `undefined` trigger defaults
