# PRD - Migracao e Unificacao CRM2 + COB2 para Rust + Dioxus

Data de criacao: 2026-02-26
Versao: v1.0
Status: Draft para execucao

## 1. Resumo executivo
Unificar os sistemas legados `crm2` e `cob2` (Django) em um unico produto reescrito em Rust, com interface em Dioxus para Web, Android e iOS, preservando regras de negocio e historico operacional.

O novo produto deve cobrir:
- Jornada de leads e vendas do CRM (captacao, atendimento, agendamento, check-in, workshop, tentativa de venda, fechamento).
- Jornada de cobranca do COB (distribuicao por workgroup, regras de atendimento por janela de dias, negociacoes, historico).
- Integracao com o `rust-sync-service` para replicacao de dados vindos do SQL Server para PostgreSQL sem dependencia de leitura online do legado.

## 2. Contexto atual

### 2.1 Sistemas origem
- `legacy/cnf-unique/crm2`: captura e tratamento de leads, agendamento e visita, check-in de convidados, workflow de venda.
- `legacy/cnf-unique/cob2`: atendimento de cobranca de associados sincronizados do legado, distribuicao por workgroups, registro de negociacoes.
- `legacy/cnf-unique/rust-sync-service`: microservico Rust que le SQL Server e escreve no PostgreSQL com regras de bloqueio/validacao.

### 2.2 Dominios comuns entre CRM2 e COB2
Apps comuns identificados:
- `accounts`, `associates`, `bills_to_receive`, `core`, `employees`, `logs`, `notifications`, `reports`, `schedules`, `settings`, `workgroups`, `workstations`.

Dominios exclusivos CRM2:
- `leads`, `sales`, `clients`, `origins`, `awards`, `indications`, `imports`, `files`.

Dominios exclusivos COB2:
- `chats`, `companies`, `databases`.

## 3. Problema
- Duplicacao de entidades e regras entre 2 projetos Django.
- Custo alto de manutencao e evolucao em duas bases separadas.
- Dependencia de dados de SQL Server instavel (intermitencia de conectividade).
- Falta de base unica para operacao omnichannel (web + mobile).

## 4. Objetivos
1. Entregar um unico produto com modelo de dados unificado para CRM + Cobranca.
2. Reescrever backend em Rust e frontend em Dioxus reaproveitando regras criticas.
3. Publicar app Web + Android + iOS com paridade funcional minima.
4. Preservar auditoria/historico e garantir rastreabilidade ponta a ponta.
5. Reduzir tempo de resposta operacional e custo de manutencao.

## 5. Nao objetivos (fase inicial)
- Reescrever o SQL Server legado.
- Eliminar o `rust-sync-service` na fase 1 (ele permanece como fonte de ingestao).
- Refatorar todas as regras para motor de regras generico antes do go-live.

## 6. Usuarios e perfis
- Atendente CRM: trata lead, agenda visita, check-in, registra tentativa de venda.
- Consultor comercial: finaliza venda, produto contratado, contrato, pulseira.
- Atendente cobranca: recebe carteira distribuida, negocia e registra historico.
- Supervisor/Coordenador: define workgroups, regras de distribuicao e acompanhamento.
- Backoffice/Financeiro: audita contratos, status e relatorios.
- TI/Operacoes: monitoramento sync, jobs, observabilidade, suporte.

## 7. Escopo funcional do produto unificado

### 7.1 Modulo CRM (Leads e Vendas)
- Captacao e cadastro de lead.
- Pipeline de atendimento e status.
- Agendamento de visitacao ao clube.
- Check-in no dia da visita (lead e convidados).
- Registro de workshop/apresentacao.
- Registro de tentativa de venda:
  - comprou ou nao,
  - tipo de pulseira,
  - produto vendido,
  - numero do contrato.
- Historico completo por lead/visita/venda.

### 7.2 Modulo Cobranca
- Cadastro e atualizacao de associados via sync.
- Distribuicao automatica/manual por workgroup e perfil.
- Regras de atendimento por faixa minima/maxima de dias de atraso.
- Registro de negociacoes e historico imutavel.
- Acompanhamento de carteira e produtividade.

### 7.3 Modulo Compartilhado
- Autenticacao/autorizacao (RBAC por perfil e workgroup).
- Configuracoes e tabelas auxiliares.
- Logs/auditoria.
- Relatorios operacionais.
- Notificacoes.

## 8. Requisitos nao funcionais
- Performance: p95 de leitura em telas criticas < 500ms (dados ja sincronizados).
- Confiabilidade: 99.5% uptime mensal para API e painel web.
- Observabilidade: logs estruturados, metricas e tracing.
- Seguranca: token/JWT, trilha de auditoria, controle de acesso por escopo.
- Mobile: mesma regra de negocio da web, com suporte offline parcial para fila local de eventos (fase 2).

## 9. Arquitetura alvo

### 9.1 Backend
- Monolito modular em Rust (API + dominio + jobs) com fronteiras por contexto:
  - `crm` (lead-to-sale),
  - `cobranca`,
  - `shared` (accounts, workgroups, logs, settings),
  - `integration` (consumo de eventos do sync).
- Banco principal: PostgreSQL.
- Redis para filas/eventos transitivos quando necessario.
- `rust-sync-service` mantido como pipeline de ingestao SQL Server -> PostgreSQL.

### 9.2 Frontend
- Dioxus Web para operacao desktop.
- Dioxus Mobile para Android/iOS compartilhando componentes e estado.
- Design system unico e navegacao por modulo.

### 9.3 Integracao de dados
- Estrategia CDC-lite via sync periodico do `rust-sync-service`.
- Proibido fluxo critico depender de consulta online direta ao SQL Server.
- Contratos de dados versionados entre sync e backend unificado.

## 10. Migracao de dados e compatibilidade
1. Catalogar tabelas equivalentes CRM2/COB2 (chaves, cardinalidade, regras).
2. Definir modelo canonico do novo sistema (schema unico).
3. Criar migradores idempotentes por dominio.
4. Executar carga inicial + reconciliacao.
5. Rodar dual-write temporario onde aplicavel (se necessario por modulo).
6. Cutover por ondas com rollback controlado.

## 11. Entregaveis
- Documento de arquitetura detalhada (ADR por decisoes chave).
- Schema unificado v1 + migracoes SQL.
- API Rust v1 (CRM + Cobranca + Shared).
- Apps Dioxus Web/Mobile v1.
- Plano de testes (unitario, integracao, E2E, regressao de regras).
- Plano de operacao (runbook, observabilidade, incidentes, rollback).

## 12. Backlog priorizado com complexidade e agente recomendado

Escala de complexidade:
- 1-2: Baixa
- 3-5: Media
- 8: Alta
- 13: Muito alta

| ID | Task/Feature | Dependencias | Complexidade | Agente recomendado |
|---|---|---|---:|---|
| T01 | Inventario completo de regras de negocio CRM2/COB2 | - | 5 | Claude |
| T02 | Mapeamento de entidades comuns e tabela canonica | T01 | 8 | BigZap |
| T03 | Definicao de arquitetura Rust modular (ADRs) | T01 | 5 | Claude |
| T04 | Esqueleto do backend Rust (workspace, modulos, CI) | T03 | 5 | Codex |
| T05 | Schema PostgreSQL unificado v1 + migracoes | T02,T04 | 8 | BigZap |
| T06 | Modulo IAM (accounts, RBAC, perfis/workgroup) | T04,T05 | 8 | Codex |
| T07 | Modulo CRM Leads (cadastro + pipeline + status) | T04,T05,T06 | 8 | Codex |
| T08 | Modulo CRM Visitacao (agenda, check-in, convidados) | T07 | 8 | Codex |
| T09 | Modulo CRM Vendas (workshop, tentativa, contrato, pulseira) | T08 | 8 | Codex |
| T10 | Modulo Cobranca carteira/distribuicao por workgroup | T04,T05,T06 | 13 | BigZap |
| T11 | Modulo Cobranca negociacoes e historico | T10 | 8 | Codex |
| T12 | Integracao com rust-sync-service (contratos e reconciliacao) | T05 | 8 | BigZap |
| T13 | API relatorios operacionais unificados | T07,T09,T11 | 5 | Claude |
| T14 | App Dioxus Web (estrutura, auth, shell, navegacao) | T06 | 5 | Codex |
| T15 | Telas Dioxus CRM (pipeline, agenda, check-in, venda) | T14,T07,T08,T09 | 13 | Codex |
| T16 | Telas Dioxus Cobranca (fila, negociacao, historico) | T14,T10,T11 | 13 | BigZap |
| T17 | App Dioxus Mobile (base compartilhada + navegacao) | T14 | 8 | Codex |
| T18 | Features mobile prioritarias (check-in + atendimento carteira) | T15,T16,T17 | 8 | Codex |
| T19 | Observabilidade (logs, metricas, tracing, dashboards) | T04 | 5 | Claude |
| T20 | Testes regressao de regras criticas (CRM + Cobranca) | T07,T08,T09,T10,T11,T12 | 8 | BigZap |
| T21 | Plano de migracao/cutover e rollback por ondas | T12,T20 | 5 | Claude |
| T22 | Go-live assistido + hiper-care | T21 | 5 | BigZap |

## 13. Timeline sugerida (ordem de execucao)

### Fase 0 - Discovery e desenho (2026-03-02 a 2026-03-20)
- Executar: T01, T02, T03.
- Marco M0: Regras mapeadas e arquitetura aprovada.

### Fase 1 - Fundacao tecnica (2026-03-23 a 2026-04-17)
- Executar: T04, T05, T06, T19.
- Marco M1: Backend base operacional + schema v1 + auth.

### Fase 2 - Dominio CRM (2026-04-20 a 2026-05-29)
- Executar: T07, T08, T09 e parte de T14.
- Marco M2: Fluxo Lead -> Agenda -> Check-in -> Venda funcional na API.

### Fase 3 - Dominio Cobranca e Integracao (2026-06-01 a 2026-07-10)
- Executar: T10, T11, T12.
- Marco M3: Carteira distribuida com regras + sync validado.

### Fase 4 - Frontend unificado web/mobile (2026-07-13 a 2026-08-21)
- Executar: T14, T15, T16, T17, T18.
- Marco M4: Web e Mobile com fluxos prioritarios completos.

### Fase 5 - Qualidade, migracao e go-live (2026-08-24 a 2026-09-25)
- Executar: T20, T21, T22.
- Marco M5: Cutover concluido e operacao estabilizada.

## 14. Criterios de aceite (alto nivel)
- Paridade funcional minima dos fluxos criticos de CRM e Cobranca.
- Nenhuma regra critica de distribuicao/bloqueio perdida na migracao.
- Historico de negociacao e atendimento auditavel.
- Sync com resiliencia a indisponibilidade do SQL Server.
- Web + Android + iOS com autenticacao, fluxos principais e logging.

## 15. Riscos e mitigacoes
- Risco: regra de negocio implicita em codigo legado nao mapeada.
  - Mitigacao: T01 com validacao por usuarios chave + testes de regressao T20.
- Risco: divergencia de dados em migracao.
  - Mitigacao: reconciliacao automatizada por lote + cutover por ondas.
- Risco: atraso no mobile por dependencias de UX e plataforma.
  - Mitigacao: priorizar vertical slice (check-in + carteira) em T18.
- Risco: sync interrompido por SQL Server fora do ar.
  - Mitigacao: arquitetura orientada ao dado sincronizado, retries e monitoracao.

## 16. Dependencias externas
- Acesso aos bancos atuais (PostgreSQL e SQL Server) para mapeamento controlado.
- Validacao de regras por especialistas de negocio (CRM e Cobranca).
- Definicao de dono de produto para priorizacao de backlog.

## 17. Definicao de pronto por task
- Especificacao funcional da task validada.
- Contratos de API/versionamento definidos.
- Casos de teste de aceite definidos.
- Dependencias desbloqueadas.

## 18. Definicao de concluido por task
- Codigo em branch com revisao aprovada.
- Testes automatizados e checks de qualidade passando.
- Telemetria minima adicionada.
- Documentacao tecnica e operacional atualizada.

