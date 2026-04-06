
## 🛠️ Plugin System — Adding New Cleaners

DevSweep uses a **JSON config plugin system**. To add support for a new project type, simply create a JSON file in the `cleaners/` directory:

### Config Schema

```json
{
  "id": "my-custom",
  "color": "#FF6B6B",
  "name": "My Framework",
  "icon": "data:image/png/base64,",
  "estimated_savings": "100MB - 500MB",
  "clean_command": "my framework clean",
  "restore_command": "my framework install",
  "description": "Clean My Framework build artifacts",
    "detect":[
        {
            "label": "Config file",
            "path": "pubspec.yaml",
            "type": "folder or file",
        },
    ],
    "clean_targets": [
        {
            "safe": true,
            "path": ".tmp",
            "type": "folder or file",
            "label": "Temporary files",
        },
    ]
}
```

### Schema Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | `string` | Unique identifier (kebab-case) |
| `name` | `string` | Display name |
| `icon` | `string` | Base64 of the image (PNG preferable) |
| `color` | `string` | Hex color for UI accent |
| `description` | `string` | Brief description |
| `detect` | `object[]` |  Array of targets that identify this project type |
| `detect[].label` | `string` |  Human-readable label |
| `detect[].path` | `string` |  Relative path of identity project  |
| `detect[].type` | `string` |  `"folder"` or `"file"` |
| `clean_targets` | `object[]` | Array of targets to clean - prioritize 'clean command' over this when available  |
| `clean_targets[].path` | `string` | Relative path to clean |
| `clean_targets[].type` | `string` | `"folder"` or `"file"` |
| `clean_targets[].label` | `string` | Human-readable label |
| `clean_targets[].safe` | `boolean` | Whether it's always safe to delete |
| `restore_command` | `string` | Command to restore after cleaning |
| `clean_command` | `string` | Command to clean project if framework already have a better cmd for it e.g 'flutter clean' |
| `estimated_savings` | `string` | Estimated disk space savings |

---
Use this link to preview based64 data (https://base64-viewer.onrender.com)[Base64 Viewer]

---
Note: Cleaner ID must be same as file (Config) name