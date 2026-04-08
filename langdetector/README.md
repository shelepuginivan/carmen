# langdetector

Service `langdetector` exposes REST API for natural language detection.


## Environment variables

| Variable                                | Description                                                 | Required | Default | Example              |
| --------------------------------------- | ----------------------------------------------------------- | -------- | ------- | -------------------- |
| `CARMEN_LANGDETECTOR_LANGUAGES`         | A JSON list of languages to detect from                     | Yes      | -       | `["en", "es", "ru"]` |
| `CARMEN_LANGDETECTOR_FALLBACK_LANGUAGE` | Language to use as a fallback if detection failed           | No       | -       | `en`                 |
| `CARMEN_LANGDETECTOR_LOW_ACCURACY`      | Increase performance at a cost of lower detection accuracy  | Yes      | `false` | `true` or `false`    |
| `CARMEN_LANGDETECTOR_PRELOAD`           | Preload language detection models to reduce initial latency | Yes      | `true`  | `true` or `false`    |


## Example Docker Compose setup

```yaml
services:
  langdetector:
    container_name: carmen-langdetector
    build:
      context: ./langdetector
    restart: unless-stopped
    environment:
      CARMEN_LANGDETECTOR_LANGUAGES: '["en", "es", "ru"]'
      CARMEN_LANGDETECTOR_FALLBACK_LANGUAGE: en
      CARMEN_LANGDETECTOR_LOW_ACCURACY: false
      CARMEN_LANGDETECTOR_PRELOAD: true
```
