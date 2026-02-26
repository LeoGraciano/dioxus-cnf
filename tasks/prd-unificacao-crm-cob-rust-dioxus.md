# PRD: Unificacao Tecnica CRM2 + COB2 em Rust + Dioxus

## 1. Introduction/Overview

Este PRD define a migracao tecnica completa dos sistemas legados `crm2` e `cob2` (Django) para uma plataforma unificada em Rust, com frontend Dioxus para Web, Android e iOS. O problema principal e manter duas bases separadas com entidades e regras duplicadas, aumentando custo de manutencao, risco de divergencia e dificuldade de evolucao.

Este documento e direcionado para execucao por agentes de IA (Codex, Claude e BigZap), com foco em arquitetura, dados, backlog tecnico, ordem de implementacao e criterios verificaveis.

## 2. Goals

- Entregar um backend Rust unificado cobrindo 100% dos fluxos tecnicos de `crm2` e `cob2`.
- Consolidar entidades compartilhadas em schema canonico unico no PostgreSQL.
- Preservar regras criticas de negocio durante migracao e reconciliacao de dados.
- Integrar com `rust-sync-service` para ingestao resiliente de dados do SQL Server.
- Disponibilizar base tecnica para Web + Android + iOS em Dioxus com contratos de API unificados.
- Permitir execucao paralela por agentes com dependencias claras entre tarefas.

## 3. User Stories

### US-001: Inventariar regras tecnicas dos legados
**Description:** Como agente de arquitetura, quero catalogar regras explicitas e implicitas de CRM2/COB2 para evitar regressao funcional durante a migracao.

**Acceptance Criteria:**
- [ ] Existe documento unico com regras por dominio (`crm`, `cobranca`, `shared`).
- [ ] Cada regra possui origem rastreavel (arquivo/tabela/fluxo legado).
- [ ] Regras com ambiguidade estao marcadas como `open-question`.
- [ ] Checklist de cobertura de regras esta >= 95% dos fluxos mapeados.

### US-002: Definir schema canonico unificado
**Description:** Como agente de dados, quero um modelo de dados unificado para remover duplicacao entre CRM2 e COB2.

**Acceptance Criteria:**
- [ ] Existe diagrama/logica de entidades com mapeamento legado -> canonico.
- [ ] Chaves unicas e relacionamentos criticos estao definidos e documentados.
- [ ] Migracoes SQL iniciais executam sem erro em ambiente limpo.
- [ ] Tabelas compartilhadas nao possuem duplicidade de responsabilidade.

### US-003: Estruturar backend Rust modular
**Description:** Como agente de backend, quero um monolito modular em Rust para implementar dominios de forma independente e previsivel.

**Acceptance Criteria:**
- [ ] Workspace Rust criado com modulos `crm`, `cobranca`, `shared`, `integration`.
- [ ] Camadas de dominio, aplicacao e infraestrutura estao separadas por contrato.
- [ ] Lint, build e testes baseline executam em CI.
- [ ] ADR de arquitetura modular aprovado.

### US-004: Implementar integracao de sync e reconciliacao
**Description:** Como agente de integracao, quero consumir dados sincronizados de forma resiliente para nao depender de leitura direta do SQL Server em fluxos criticos.

**Acceptance Criteria:**
- [ ] Contrato de integracao com `rust-sync-service` esta versionado.
- [ ] Processo de reconciliacao detecta divergencias por lote e gera relatorio.
- [ ] Falha de sincronizacao nao interrompe operacao de leitura no sistema unificado.
- [ ] Existe procedimento tecnico de reprocessamento idempotente.

### US-005: Migrar dominio CRM completo
**Description:** Como agente de produto tecnico, quero migrar todos os fluxos tecnicos de CRM (lead ate venda) para manter paridade completa.

**Acceptance Criteria:**
- [ ] Endpoints de lead, pipeline, agendamento e check-in implementados.
- [ ] Endpoints de workshop, tentativa de venda, contrato e pulseira implementados.
- [ ] Historico tecnico de eventos do lead fica auditavel ponta a ponta.
- [ ] Testes de regressao do dominio CRM aprovados.

### US-006: Migrar dominio Cobranca completo
**Description:** Como agente de cobranca, quero migrar distribuicao de carteira e negociacoes para manter operacao atual sem perda de regra.

**Acceptance Criteria:**
- [ ] Regras de distribuicao por workgroup/perfil implementadas.
- [ ] Regras de janela minima/maxima de atendimento implementadas.
- [ ] Registro de negociacoes e historico imutavel implementados.
- [ ] Testes de regressao do dominio Cobranca aprovados.

### US-007: Preparar cutover por ondas
**Description:** Como agente de operacao, quero migrar em ondas controladas para reduzir risco de indisponibilidade e rollback complexo.

**Acceptance Criteria:**
- [ ] Plano de ondas com criterio de entrada/saida por modulo definido.
- [ ] Cada onda possui checklist de validacao tecnica pre e pos-cutover.
- [ ] Rollback por onda e testado em ambiente de homologacao.
- [ ] Nenhuma etapa depende de migracao big bang.

## 4. Functional Requirements

- FR-1: O sistema deve consolidar entidades comuns de `crm2` e `cob2` em schema canonico unico no PostgreSQL.
- FR-2: O backend deve ser implementado em Rust com modulos separados por contexto (`crm`, `cobranca`, `shared`, `integration`).
- FR-3: O modulo `crm` deve suportar cadastro de lead, pipeline de status, agendamento, check-in de visita e registro de venda.
- FR-4: O modulo `cobranca` deve suportar distribuicao de carteira por workgroup, regras de janela de atraso e registro de negociacoes.
- FR-5: O sistema deve manter trilha de auditoria tecnica para eventos criticos de CRM e Cobranca.
- FR-6: A integracao com `rust-sync-service` deve usar contrato versionado e reconciliacao idempotente.
- FR-7: Fluxos criticos nao devem depender de consulta online direta ao SQL Server.
- FR-8: O frontend em Dioxus deve consumir APIs unificadas para Web, Android e iOS.
- FR-9: O processo de migracao deve ocorrer por ondas com criterio objetivo de avancar/reverter.
- FR-10: Cada task tecnica deve ter dependencia explicita, complexidade e agente responsavel para execucao coordenada.

## 5. Non-Goals (Out of Scope)

- Nao realizar migracao em modelo big bang (corte unico total).

## 6. Design Considerations (Optional)

- Reusar contratos e nomenclaturas de dominio para minimizar ambiguidade entre agentes.
- Padronizar naming entre entidades de CRM e Cobranca antes de construir telas.
- Priorizar consistencia estrutural de API sobre customizacao visual na primeira etapa.

## 7. Technical Considerations (Optional)

- Dependencia critica: `rust-sync-service` como fonte de ingestao SQL Server -> PostgreSQL.
- Restricao operacional: SQL Server pode ficar indisponivel; sistema novo deve operar sobre dados sincronizados.
- Banco alvo: PostgreSQL com migracoes versionadas e scripts idempotentes.
- Observabilidade minima: logs estruturados, metricas de sync e tracing por fluxo.
- Testes obrigatorios: unitarios, integracao, regressao de regras e validacao de reconciliacao.

## 8. Success Metrics

- 100% dos fluxos tecnicos de CRM2 e COB2 mapeados para modulos do sistema unificado.
- >= 95% de cobertura de regras criticas validadas em testes de regressao.
- 0 dependencia de leitura direta do SQL Server em fluxos criticos de atendimento.
- 100% das ondas de migracao com checklist de validacao e rollback documentados.
- Tempo medio de reconciliacao por lote dentro da janela operacional definida pelo time (SLO interno).

## 9. Open Questions

- Qual sera a janela operacional exata (horarios e duracao) para cada onda de cutover?
- Quais campos legados terao deprecacao planejada no schema canonico v2?
- Quais limites de SLO/SLA devem ser formalizados para API e sync na fase de go-live?
- Qual estrategia final para versionamento de contratos API entre backend Rust e apps Dioxus?

