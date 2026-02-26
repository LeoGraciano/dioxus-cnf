# Django Template Support in Zed

## Issue
Zed doesn't recognize Django templates (`.html` files with `{% %}` and `{{ }}` tags) natively.

## Solutions

### 1. Official Extension (Recommended)
Install the official Django extension from Zed:
1. Open Zed
2. Press `Cmd+Shift+X` (or `Ctrl+Shift+X`)
3. Search for "Django"
4. Install the extension by [joshuadavidthomas](https://github.com/joshuadavithomas/zed-django)

### 2. Current Configuration
The current `.zed/settings.json` treats template files as regular HTML, which provides basic syntax highlighting but no Django-specific features.

### 3. Alternative Approach
For better Django template support:
- Use the extension mentioned above
- Or open templates as plain text to avoid unwanted formatting
- Or use VSCode for template editing if needed

## Status
- ✅ Basic HTML syntax highlighting works
- ❌ Django template tag recognition (needs extension)
- ❌ Django auto-completion (needs extension)