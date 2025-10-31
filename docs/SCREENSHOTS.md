# Screenshot Guide

This guide explains what screenshots to capture for the PostgreSQL SQL features documentation.

## Required Screenshots

### 1. Main Table List View
**File**: `table-list.png`
**Description**: Shows the sidebar with schema browser, expandable tables, and column information
**What to capture**:
- Sidebar with database connections
- Expandable table list showing schemas and tables
- Expanded table showing columns with metadata (PK, FK, NOT NULL chips)
- Column types and foreign key relationships visible

### 2. Table Data View
**File**: `table-data.png`
**Description**: Shows the main table view with data, filters, and pagination
**What to capture**:
- Table with rows of data
- Column headers showing metadata (PK, FK chips)
- Filter row with different input types
- Pagination controls at the bottom
- Row count display

### 3. Foreign Key Navigation
**File**: `foreign-key-navigation.png`
**Description**: Shows clicking on a foreign key value
**What to capture**:
- A table row with a foreign key column visible
- The foreign key value styled as a clickable link
- Tooltip showing the referenced table information
- Cursor hovering over the foreign key link

### 4. Filter UI
**File**: `filter-ui.png`
**Description**: Shows different filter input types based on column data types
**What to capture**:
- Text input for text columns
- Number input for numeric columns
- Date/time picker for temporal columns
- Dropdown select for ENUM columns
- Checkbox for boolean columns
- Filter operators visible in tooltips

### 5. Column Metadata Tooltips
**File**: `column-tooltips.png`
**Description**: Shows rich metadata displayed on column hover
**What to capture**:
- Tooltip showing:
  - Data type
  - Nullability
  - Primary key status
  - Foreign key relationships
  - Default values
  - ENUM allowed values (if applicable)

### 6. Foreign Key Filter Indicator
**File**: `fk-filter-banner.png`
**Description**: Shows the banner indicating filtered view from foreign key navigation
**What to capture**:
- Info alert banner at the top of table view
- Message showing which column is filtered
- Filter value displayed
- Clear button visible
- Highlighted row matching the filter

### 7. Highlighted Row
**File**: `highlighted-row.png`
**Description**: Shows the highlighted row matching the foreign key filter
**What to capture**:
- Table with a highlighted row (blue background)
- Left border accent on the highlighted row
- Multiple rows visible to show the highlighting contrast
- Foreign key filter banner visible at the top

## How to Capture Screenshots

1. Start the development server:
   ```bash
   cargo run --example axum_server
   ```

2. Navigate to `http://localhost:3000/dev/ui`

3. Connect to a PostgreSQL database with sample data

4. Capture screenshots using:
   - **macOS**: `Cmd + Shift + 4` (select area) or `Cmd + Shift + 3` (full screen)
   - **Windows**: `Win + Shift + S` (Snipping Tool)
   - **Linux**: Use `gnome-screenshot` or `scrot`

5. Save screenshots in `docs/screenshots/` directory

6. Update `SQL_POSTGRES_FEATURES.md` with the screenshot references

## Recommended Tools

- **Annotate screenshots**: Use tools like:
  - **macOS**: Preview, Skitch
  - **Windows**: Snip & Sketch, Greenshot
  - **Linux**: Flameshot, Shutter
  - **Cross-platform**: Lightshot, ShareX

- **Resize/optimize**: Use tools like:
  - ImageOptim (macOS)
  - TinyPNG (web)
  - ImageMagick (command line)

## File Naming Convention

- Use lowercase with hyphens: `table-list.png`, `foreign-key-navigation.png`
- Keep file names descriptive but concise
- Use PNG format for better quality
- Optimize images before committing (aim for < 500KB per image)

