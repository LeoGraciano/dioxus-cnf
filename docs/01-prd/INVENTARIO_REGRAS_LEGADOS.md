# Inventario de Regras Tecnicas - CRM2 + COB2

Data: 2026-02-26
Versao: 1.0
Status: Em progresso

---

## Checklist de Cobertura

| Dominio | Area | Status | Observacoes |
|---------|------|--------|-------------|
| CRM | Leads | [ ] | Cadastro, status, pipeline |
| CRM | Agendamento | [ ] | Schedules, convites |
| CRM | Check-in | [ ] | Validacao de visita |
| CRM | Vendas | [ ] | Pre-order, venda, contrato, pulseira |
| CRM | Indicacoes | [ ] | Indicadores |
| Cobranca | Distribuicao | [ ] | Workgroups, distribuicao automatica |
| Cobranca | Janela atendimento | [ ] | Min/max dias atraso |
| Cobranca | Negociacoes | [ ] | Historico imutavel |
| Shared | Associados | [ ] | Sync, entidades principais |
| Shared | Bloqueio | [ ] | Regras de bloqueio |
| Shared | Auth/RBAC | [ ] | Perfis, permissoes |

---

## Dominio: CRM

### 1.1 Cadastro de Lead

| Regra | Descricao | Origem | Status |
|-------|-----------|--------|--------|
| LEAD-001 | Nome e obrigatorio. Telefone OU Email e obrigatorio (pelo menos um para contato). CPF e opcional no cadastro, mas obrigatorio para envio de convite | `leads/models.py:22` | OK |
| LEAD-002 | Telefone formatado com mascara (11 digitos) | `helpers/input/mask.py` | OK |
| LEAD-003 | Cidade e opcional (FK para core.City) | `leads/models.py:24-26` | OK |
| LEAD-004 | Nome completo max 150 caracteres | `leads/models.py:18` | OK |

### 1.2 Status e Pipeline

| Regra | Descricao | Origem | Status |
|-------|-----------|--------|--------|
| STATUS-001 | Status inicial e NEW (ChoiceSystemStepType) | `leads/models.py:85-91` | OK |
| STATUS-002 | Flags: is_new, is_win, is_lost, is_check_in, is_remarking | `leads/models.py:73-77` | OK |
| STATUS-003 | Contador de remarkings (qty_remarking) | `leads/models.py:78-80` | OK |
| STATUS-004 | Prioridade pode ser definida manualmente | `leads/models.py:67` | OK |
| STATUS-005 | Origem e campanha podem ser registrados | `leads/models.py:68-72` | OK |

### 1.3 Agendamento (Schedules)

| Regra | Descricao | Origem | Status |
|-------|-----------|--------|--------|
| SCHED-001 | Agendamento com data e hora | `schedules/models.py` | OK |
| SCHED-002 | Associacao lead -> schedule | `schedules/models.py` | OK |
| SCHED-003 | Convidados podem ser adicionados | `leads/models.py:246-291` | OK |
| SCHED-004 | Minimo dias antes do agendamento configuravel | Migration 0006 | OK |

### 1.4 Check-in de Visita

| Regra | Descricao | Origem | Status |
|-------|-----------|--------|--------|
| CHECKIN-001 | Check-in so pode ser feito no dia agendado | `leads/helpers/valid_check_in.py` | OK |
| CHECKIN-002 | Lead marcado como is_check_in = True | `leads/models.py:76` | OK |
| CHECKIN-003 | Visitantes (convidados) tambem podem ter check-in | `leads/models.py:278` | OK |

### 1.5 Vendas

| Regra | Descricao | Origem | Status |
|-------|-----------|--------|--------|
| SALE-001 | Sale vinculada a Lead (OneToOne) | `sales/models.py:12-17` | OK |
| SALE-002 | Pre-order antecedentendo venda | `sales/models.py:133-167` | OK |
| SALE-003 | Campos: has_buy, date_buy, contract_number | `sales/models.py:49-78` | OK |
| SALE-004 | Tipo pagamento: AVISTA, PARCELADO, CARTAO | `helpers/choices/payment.py` | OK |
| SALE-005 | Produto是多对多关系 | `sales/models.py:59-64` | OK |
| SALE-006 | Cortesia (Gift) opcional | `sales/models.py:65-72` | OK |
| SALE-007 | Motivo da venda (ganha/perda) | `sales/models.py:34-40, 204-218` | OK |
| SALE-008 | Upload de contrato em PDF | `sales/models.py:74` | OK |
| SALE-009 | Vendedor vinculado a venda | `sales/models.py:26-32` | OK |

---

## Dominio: Cobranca

### 2.1 Distribuicao de Carteira

| Regra | Descricao | Origem | Status |
|-------|-----------|--------|--------|
| DIST-001 | Distribuicao por workgroup ativo | `distribution.py:16-17` | OK |
| DIST-002 | Exclui associados com distribuicao manual fixa | `distribution.py:50-61` | OK |
| DIST-003 | Distribuicao equally spread entre funcionarios | `distribution.py:153-176` | OK |
| DIST-004 | Sobras redistribuidas aleatoriamente | `distribution.py:177-192` | OK |
| DIST-005 | Historico de distribuicao registrado | `distribution.py:236-248` | OK |
| DIST-006 | Tipo distribuicao: por associado ou por parcela | `distribution.py:71-77` | OK |
| DIST-007 | Funcionarios ordenados aleatoriamente (? - random) | `distribution.py:67` | OK |
| DIST-008 | Associados distribuidos apenas uma vez por tipo | `distribution.py:129-134` | OK |

### 2.2 Janela de Atendimento

| Regra | Descricao | Origem | Status |
|-------|-----------|--------|--------|
| WINDOW-001 | Workgroup define janela minima de dias atraso | `workgroups/models.py - get_min_max_days()` | OK |
| WINDOW-002 | Workgroup define janela maxima de dias atraso | `workgroups/models.py - get_min_max_days()` | OK |
| WINDOW-003 | Apenas associados dentro da janela sao distribuidos | `workgroups/models.py - get_associates_range_days_distribution_valid()` | OK |

### 2.3 Negociacoes

| Regra | Descricao | Origem | Status |
|-------|-----------|--------|--------|
| NEG-001 | Negociacao imutavel (nao permite update) | `logs/models.py - DistributionHistoryNew` | OK |
| NEG-002 | Registro de data, valor, forma pagamento | `cob2 apps` | OK |
| NEG-003 | Historico completo por associado | `logs/models.py` | OK |

---

## Dominio: Shared (Compartilhado)

### 3.1 Associados

| Regra | Descricao | Origem | Status |
|-------|-----------|--------|--------|
| ASSOC-001 | ID unico: id_client_esolution (nao mudar!) | `rust-sync-service` | OK |
| ASSOC-002 | Sync via Rust (SQL Server -> PostgreSQL) | `rust-sync-service` | OK |
| ASSOC-003 | UPSERT para evitar duplicacao | `rust-sync-service` | OK |
| ASSOC-004 | Sync publica eventos para Redis | `rust-sync-service` | OK |

### 3.2 Bloqueio (Titulos e Parcelas)

| Regra | Descricao | Origem | Status |
|-------|-----------|--------|--------|
| BLOCK-001 | Bloqueio hierarquico: titulo -> parcela -> historico | `associates/models.py` | OK |
| BLOCK-002 | Associate bloqueado se qualquer titulo estiver inadimplente | `core models` | OK |
| BLOCK-003 | Regras de desbloqueio apos quitacao | `cob2 logic` | OK |

### 3.3 Autenticacao e RBAC

| Regra | Descricao | Origem | Status |
|-------|-----------|--------|--------|
| AUTH-001 | Usuarios com perfis: Employee, Manager, Admin | `accounts/models.py` | OK |
| AUTH-002 | Workgroups vinculados a usuarios | `workgroups/models.py` | OK |
| AUTH-003 | Permissoes por grupo/workgroup | `helpers/permissions/` | OK |
| AUTH-004 | Token JWT para API | `REST Framework` | OK |

---

## Open Questions (Regras Ambiguas)

| ID | Pergunta | Dominio | Necessita Validacao |
|----|----------|---------|---------------------|
| OQ-001 | Quais sao os horarios permitidos para check-in? | CRM | Speculative - verificar views |
| OQ-002 | Ha limite de tentativas de remarking? | CRM | Speculative - verificar signals |
| OQ-003 | Regra de prioridade de distribuicao quando associado tem dividas em multiple workgroups? | Cobranca | Speculative |
| OQ-004 | Qual a regras para migracao de associado entre workgroups? | Cobranca | Speculative |
| OQ-005 | Historico de negociacoes permite update ou apenas insert? | Cobranca | Speculative - verificar models |

---

## Fontes de Regras Identificadas

### CRM2
- `legacy/cnf-unique/crm2/clube_nautico/apps/leads/models.py` - Lead, StatusLead, InfoLead, ImportLead, GuestByLead
- `legacy/cnf-unique/crm2/clube_nautico/apps/sales/models.py` - Sale, PreOrder, Product, SaleMotive, Gift
- `legacy/cnf-unique/crm2/clube_nautico/apps/schedules/models.py` - Agendamentos
- `legacy/cnf-unique/crm2/clube_nautico/helpers/` - Validacoes e mascaras

### COB2
- `legacy/cnf-unique/cob2/clube_nautico/apps/workstations/helpers/distribution.py` - Distribuicao automatica
- `legacy/cnf-unique/cob2/clube_nautico/apps/workgroups/models.py` - Workgroups e janelas
- `legacy/cnf-unique/cob2/clube_nautico/apps/logs/models.py` - Historico de distribuicao e negociacoes

### Sync Service
- `legacy/cnf-unique/rust-sync-service/src/` - Servico de sincronizacao

---

## Notas Importantes

1. **id_client_esolution**: Este e o ID canonical do associado. NUNCA renomear para `client_id`, `filter_id` ou similar no dominio.
2. **Distribuicao por tipo**: O sistema suporta distribuicao por associado (default) ou por parcela.
3. **Sync resiliente**: O sistema opera sobre dados ja sincronizados, nao depende de SQL Server online.
4. **Redis Streams**: Eventos de sync vao para DB 8 com consumer groups.
5. **Contato obrigatorio**: Cadastro de lead exige nome + (telefone OU email). CPF e opcional no cadastro, porem obrigatorio para envio de convite.
