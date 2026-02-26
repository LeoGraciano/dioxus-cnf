# MCP (Model Context Protocol)

Arquivo principal: `mcp.json` na raiz do repo.

Os agentes devem sempre ler o `mcp.json` da raiz para contexto de Rust, Django e Dioxus.

Resumo:
- Frontend Dioxus (workspace): `apps/frontend`
- Pacotes: `apps/frontend/packages/{web,mobile,desktop,ui}`
- Workspace Rust (raiz): `Cargo.toml`
- Legado Django: `legacy/cnf-unique` (crm2/cob2)
