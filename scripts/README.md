# Scripts

Estrutura padronizada por categoria:

- `scripts/dev/`:
  - `run-dev.sh`: inicia hot reload (Dioxus)
- `scripts/prod/`:
  - `run-prod.sh`: builds de release (web/server/android/ios/desktop)
- `scripts/ci/`:
  - `run-ci.sh`: espelha o build do CI localmente
- `scripts/deploy/`:
  - `run-web.sh`: stub deploy web
  - `run-android.sh`: stub deploy android
  - `run-ios.sh`: stub deploy ios
  - `run-desktop.sh`: stub deploy desktop

Uso:

```bash
scripts/dev/run-dev.sh web
scripts/dev/run-dev.sh mobile
scripts/dev/run-dev.sh desktop
```

```bash
scripts/prod/run-prod.sh all
scripts/prod/run-prod.sh web
scripts/prod/run-prod.sh server
scripts/prod/run-prod.sh android
scripts/prod/run-prod.sh ios
scripts/prod/run-prod.sh desktop
```

```bash
scripts/ci/run-ci.sh
```
