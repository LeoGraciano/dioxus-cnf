# Schema Canonico Unificado - CRM + Cobranca

Data: 2026-02-26
Versao: 1.0
Status: Em progresso
Projeto: Unificacao CRM2 + COB2 em Rust + Dioxus

---

## Visao Geral

Este documento define o schema canonico unificado que ira substituir os modelos de dados separados do CRM2 e COB2. O objetivo e eliminar duplicacao de entidades e definir um modelo de dados unico que suporte ambos os dominios.

---

## 1. Mapeamento Entidade Legado -> Canonico

### Dominio: Shared (Compartilhado)

| Entidade Canonica | CRM2 (tb_*) | COB2 (tb_*) | Observacoes |
|-------------------|-------------|-------------|-------------|
| associate | associates_associate | associates_associate | Tabela unificada via sync |
| associate_contact | core_contact | associates_contact | Phone/email |
| associate_address | core_address | associates_address | Endereco |
| city | core_city | core_city | Tabela compartilhada |
| state | core_state | core_state | Tabela compartilhada |
| user | accounts_user | accounts_user | Unificado |
| user_employee | accounts_useremployee | employees_employee | Unificado |
| user_client | accounts_userclient | - | Apenas CRM |
| workgroup | workgroups_workgroup | workgroups_workgroup | Unificado |
| workstation | workstations_workstation | workstations_workstation | Unificado |
| workstation_associate | workstations_associate | workstations_associate | Carteira de cobranca |
| log_audit | logs_* | logs_* | Auditoria via simple_history |

### Dominio: CRM (Leads e Vendas)

| Entidade Canonica | CRM2 (tb_*) | Legacy Table | Observacoes |
|-------------------|-------------|--------------|-------------|
| lead | tb_lead | leads_lead | Cadastro base |
| lead_status | tb_status_lead | leads_statuslead | Status e flags |
| lead_info | tb_info_lead | leads_infolead | Info adicional |
| lead_import | tb_import_lead | leads_importlead | Origem captura |
| guest | tb_guest_by_lead | leads_guestbylead | Visitantes |
| schedule | - | schedules_schedule | Agendamentos |
| schedule_associate | - | schedules_scheduleassociate | Vinculo lead->agenda |
| sale | tb_sale | sales_sale | Venda final |
| pre_order | tb_pre_order | sales_preorder | Pre-venda |
| product | tb_product | sales_product | Produtos |
| sale_motive | tb_sale_motive | sales_salemotive | Motivo ganha/perda |
| gift | tb_gift | sales_g |
| origin |ift | Cortesia tb_origin | origins_origin | Origens marketing |
| indication | tb_indication | indications_indication | Indicacoes |
| file | tb_file | files_file | Arquivos uploads |

### Dominio: Cobranca

| Entidade Canonica | COB2 (tb_*) | Legacy Table | Observacoes |
|-------------------|-------------|--------------|-------------|
| title | tb_title | bills_to_receive_title | Titulo financeiro |
| installment | tb_installment | bills_to_receive_installment | Parcela |
| workgroup_config | tb_workgroup | workgroups_workgroup | Config janela |
| distribution_log | tb_distribution_history | logs_distributionhistorynew | Log distribuicao |
| negotiation | tb_negotiation | logs_negotiation | Negociacoes |

---

## 2. Chaves Unicas e Relacionamentos Criticos

### 2.1 Entidades Principais

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              ASSOCIATE                                   │
│  PK: id_client_esolution (UUID) - NUNCA RENOMEAR!                      │
│  UK: cpf (nullable)                                                     │
│  UK: email (nullable)                                                  │
└─────────────────────────────────────────────────────────────────────────┘
       │
       ├─► associate_contact (1:N) - telefones, emails
       ├─► associate_address (1:N) - enderecos
       ├─► title (1:N) - titulos financeiros
       ├─► workstation_associate (1:N) - carteiras
       ├─► guest (1:N) - visitantes de lead
       └─► negotiation (1:N) - historico negociacoes

┌─────────────────────────────────────────────────────────────────────────┐
│                                 LEAD                                    │
│  PK: id (UUID)                                                         │
│  UK: email (obrigatorio)                                               │
│  UK: cpf (nullable)                                                    │
└─────────────────────────────────────────────────────────────────────────┘
       │
       ├─► lead_status (1:1) - status, flags, prioridade
       ├─► lead_info (1:1) - info adicional (niver, estado civil)
       ├─► lead_import (1:1) - origem da captura
       ├─► guest (1:N) - convidados
       ├─► schedule (1:N) - agendamentos
       └─► sale (1:1) - venda final

┌─────────────────────────────────────────────────────────────────────────┐
│                               WORKGROUP                                 │
│  PK: id (UUID)                                                         │
│  UK: name                                                              │
│  FK: employee_id (owner)                                               │
└─────────────────────────────────────────────────────────────────────────┘
       │
       ├─► workstation (1:N) - estacoes de trabalho
       └─► workgroup_config (1:1) - config janelas min/max

┌─────────────────────────────────────────────────────────────────────────┐
│                             WORKSTATION                                 │
│  PK: id (UUID)                                                         │
│  FK: workgroup_id                                                      │
│  FK: employee_id (atendente)                                          │
└─────────────────────────────────────────────────────────────────────────┘
       │
       └─► workstation_associate (1:N) - carteira distribuida

┌─────────────────────────────────────────────────────────────────────────┐
│                                TITLE                                    │
│  PK: id (UUID)                                                         │
│  FK: associate_id (id_client_esolution)                               │
│  UK: document_number                                                  │
└─────────────────────────────────────────────────────────────────────────┘
       │
       └─► installment (1:N) - parcelas

┌─────────────────────────────────────────────────────────────────────────┐
│                              INSTALLMENT                                │
│  PK: id (UUID)                                                         │
│  FK: title_id                                                          │
│  FK: associate_id (id_client_esolution)                               │
│  UK: installment_number + title_id                                    │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│                                SALE                                     │
│  PK: id (UUID)                                                         │
│  FK: lead_id                                                           │
│  FK: employee_id (vendedor)                                            │
│  UK: contract_number (nullable)                                       │
└─────────────────────────────────────────────────────────────────────────┘
       │
       ├─► product (N:M) - produtos vendidos
       └─► gift (1:N) - cortestias
```

### 2.2 Chaves Unicas por Entidade

| Entidade | Chave Unica | Tipo |
|----------|-------------|------|
| associate | id_client_esolution | UUID (PK) |
| associate | cpf | VARCHAR(11) |
| associate | email | VARCHAR(255) |
| lead | id | UUID (PK) |
| lead | email | VARCHAR(255) |
| lead | cpf | VARCHAR(11) |
| workgroup | id | UUID (PK) |
| workgroup | name | VARCHAR(100) |
| workstation | id | UUID (PK) |
| workstation | workgroup_id + employee_id | UK |
| title | id | UUID (PK) |
| title | associate_id + document_number | UK |
| installment | id | UUID (PK) |
| installment | title_id + installment_number | UK |
| sale | id | UUID (PK) |
| sale | lead_id | UK (1:1) |
| sale | contract_number | VARCHAR(50) |

---

## 3. Responsabilidade de Tabelas Compartilhadas

### 3.1 Tabelas que NAO tem duplicacao

| Tabela | CRM2 | COB2 | Status | Acao |
|--------|------|------|--------|------|
| core_city | sim | sim | OK | Compartilhar |
| core_state | sim | sim | OK | Compartilhar |
| accounts_user | sim | sim | OK | Compartilhar |
| workgroups_workgroup | sim | sim | OK | Compartilhar |
| workstations_workstation | sim | sim | OK | Compartilhar |
| logs_* | sim | sim | OK | Compartilhar via simple_history |

### 3.2 Tabelas com Duplicacao (Requierem Consolidacao)

| Tabela | CRM2 | COB2 | Solucao Proposta |
|--------|------|------|------------------|
| associates_associate | sim | sim | Unificar - mesma tabela, sync via rust-sync-service |
| associates_contact | - | sim | Unificar em associate_contact |
| associates_address | - | sim | Unificar em associate_address |
| core_contact | sim | - | Migrar para associate_contact |
| core_address | sim | - | Migrar para associate_address |
| accounts_useremployee | sim | - | Unificar com employees_employee |
| employees_employee | - | sim | Unificar com accounts_useremployee |
| bills_to_receive_* | sim | sim | Consolidar em title/installment |

---

## 4. Migracoes SQL Iniciais

### 4.1 Tabelas de Dominio Shared

```sql
-- Tabela principal de associados (unificada)
CREATE TABLE associate (
    id_client_esolution UUID PRIMARY KEY,
    cpf VARCHAR(11),
    name VARCHAR(150) NOT NULL,
    email VARCHAR(255),
    phone_number VARCHAR(11),
    birth_date DATE,
    is_blocked BOOLEAN DEFAULT FALSE,
    blocked_reason TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_associate_cpf ON associate(cpf) WHERE cpf IS NOT NULL;
CREATE INDEX idx_associate_email ON associate(email) WHERE email IS NOT NULL;
CREATE UNIQUE INDEX idx_associate_cpf_unique ON associate(cpf) WHERE cpf IS NOT NULL;

-- Contatos do associado (telefones, emails)
CREATE TABLE associate_contact (
    id UUID PRIMARY KEY,
    associate_id UUID NOT NULL REFERENCES associate(id_client_esolution),
    contact_type VARCHAR(20) NOT NULL, -- 'phone', 'email', 'whatsapp'
    value VARCHAR(255) NOT NULL,
    is_primary BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_associate_contact_associate ON associate_contact(associate_id);

-- Enderecos do associado
CREATE TABLE associate_address (
    id UUID PRIMARY KEY,
    associate_id UUID NOT NULL REFERENCES associate(id_client_esolution),
    address_type VARCHAR(20) NOT NULL, -- 'residential', 'commercial'
    street VARCHAR(255),
    number VARCHAR(20),
    complement VARCHAR(100),
    district VARCHAR(100),
    city VARCHAR(100),
    state VARCHAR(2),
    zip_code VARCHAR(10),
    is_primary BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_associate_address_associate ON associate_address(associate_id);

-- Cidades (compartilhada)
CREATE TABLE city (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    state_id VARCHAR(2) NOT NULL,
    id_api INTEGER
);

CREATE INDEX idx_city_state ON city(state_id);

-- Estados (compartilhado)
CREATE TABLE state (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    uf VARCHAR(2) NOT NULL
);

-- Workgroups (compartilhado)
CREATE TABLE workgroup (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    min_days_delay INTEGER DEFAULT 0,
    max_days_delay INTEGER DEFAULT 999,
    owner_id UUID REFERENCES associate(id_client_esolution),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_workgroup_name ON workgroup(name);

-- Estacoes de trabalho (atendentes por workgroup)
CREATE TABLE workstation (
    id UUID PRIMARY KEY,
    workgroup_id UUID NOT NULL REFERENCES workgroup(id),
    employee_id UUID NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_workstation_workgroup_employee ON workstation(workgroup_id, employee_id);

-- Carteira de cobranca (associaldos distribuidos)
CREATE TABLE workstation_associate (
    id UUID PRIMARY KEY,
    workstation_id UUID NOT NULL REFERENCES workstation(id),
    associate_id UUID NOT NULL REFERENCES associate(id_client_esolution),
    distribution_type VARCHAR(20) NOT NULL, -- 'associate', 'installment'
    distributed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_workstation_associate_unique ON workstation_associate(workstation_id, associate_id, distribution_type);
CREATE INDEX idx_workstation_associate_associate ON workstation_associate(associate_id);
```

### 4.2 Tabelas de Dominio CRM

```sql
-- Leads (cadastro base)
CREATE TABLE lead (
    id UUID PRIMARY KEY,
    name VARCHAR(150) NOT NULL,
    cpf VARCHAR(11),
    email VARCHAR(255) NOT NULL,
    phone_number VARCHAR(11),
    city_id UUID REFERENCES city(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_lead_email ON lead(email);
CREATE INDEX idx_lead_cpf ON lead(cpf) WHERE cpf IS NOT NULL;

-- Status do lead (flags, prioridade)
CREATE TABLE lead_status (
    id UUID PRIMARY KEY,
    lead_id UUID NOT NULL REFERENCES lead(id) ON DELETE CASCADE,
    priority INTEGER DEFAULT 0,
    origin VARCHAR(60),
    campaign VARCHAR(255),
    tag VARCHAR(255),
    is_new BOOLEAN DEFAULT TRUE,
    is_win BOOLEAN DEFAULT FALSE,
    is_lost BOOLEAN DEFAULT FALSE,
    is_check_in BOOLEAN DEFAULT FALSE,
    is_remarking BOOLEAN DEFAULT FALSE,
    qty_remarking SMALLINT DEFAULT 0,
    status_attendance VARCHAR(250) DEFAULT 'NEW',
    registed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_lead_status_lead ON lead_status(lead_id);

-- Info adicional do lead
CREATE TABLE lead_info (
    id UUID PRIMARY KEY,
    lead_id UUID NOT NULL REFERENCES lead(id) ON DELETE CASCADE,
    marital_status VARCHAR(100),
    phone_number2 VARCHAR(11),
    phone_number3 VARCHAR(11),
    birthday DATE,
    sons VARCHAR(10),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_lead_info_lead ON lead_info(lead_id);

-- Origem da captura do lead
CREATE TABLE lead_import (
    id UUID PRIMARY KEY,
    lead_id UUID NOT NULL REFERENCES lead(id) ON DELETE CASCADE,
    import_type VARCHAR(50), -- 'google_sheets', 'spreadsheet', 'employee_register', 'migration'
    import_source_id UUID,
    employee_register_id UUID,
_id UUID,
    created_at TIMESTAMP WITH TIME ZONE    refer DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_lead_import_lead ON lead_import(lead_id);

-- Convidados do lead (visitas)
CREATE TABLE guest (
    id UUID PRIMARY KEY,
    lead_id UUID NOT NULL REFERENCES lead(id) ON DELETE CASCADE,
    name VARCHAR(150),
    cpf VARCHAR(11),
    age SMALLINT,
    phone_number VARCHAR(11),
    city VARCHAR(150),
    state VARCHAR(2),
    kinship VARCHAR(50),
    is_visitor BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_guest_lead ON guest(lead_id);

-- Agendamentos de visita
CREATE TABLE schedule (
    id UUID PRIMARY KEY,
    lead_id UUID REFERENCES lead(id) ON DELETE SET NULL,
    scheduled_date DATE NOT NULL,
    scheduled_time TIME NOT NULL,
    min_days_before INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_schedule_lead ON schedule(lead_id);
CREATE INDEX idx_schedule_date ON schedule(scheduled_date);

-- Vendas
CREATE TABLE sale (
    id UUID PRIMARY KEY,
    lead_id UUID NOT NULL REFERENCES lead(id),
    employee_id UUID NOT NULL,
    has_buy BOOLEAN DEFAULT FALSE,
    date_buy DATE,
    contract_number VARCHAR(50),
    payment_type VARCHAR(20), -- 'AVISTA', 'PARCELADO', 'CARTAO'
    bracelet_type VARCHAR(50),
    sale_motive_id UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_sale_lead ON sale(lead_id);
CREATE INDEX idx_sale_contract ON sale(contract_number) WHERE contract_number IS NOT NULL;

-- Pre-orders (vendas antecipadas)
CREATE TABLE pre_order (
    id UUID PRIMARY KEY,
    lead_id UUID NOT NULL REFERENCES lead(id),
    sale_id UUID REFERENCES sale(id),
    product_id UUID,
    quantity INTEGER DEFAULT 1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Produtos
CREATE TABLE product (
    id UUID PRIMARY KEY,
    name VARCHAR(150) NOT NULL,
    description TEXT,
    price DECIMAL(10,2),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Motivos de venda (ganha/perda)
CREATE TABLE sale_motive (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    is_win BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Cortesias
CREATE TABLE gift (
    id UUID PRIMARY KEY,
    sale_id UUID NOT NULL REFERENCES sale(id),
    name VARCHAR(150),
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Origens marketing
CREATE TABLE origin (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indicacoes
CREATE TABLE indication (
    id UUID PRIMARY KEY,
    lead_id UUID NOT NULL REFERENCES lead(id),
    indicato_id UUID REFERENCES associate(id_client_esolution),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### 4.3 Tabelas de Dominio Cobranca

```sql
-- Titulos financeiros
CREATE TABLE title (
    id UUID PRIMARY KEY,
    associate_id UUID NOT NULL REFERENCES associate(id_client_esolution),
    document_number VARCHAR(50) NOT NULL,
    title_date DATE,
    total_value DECIMAL(12,2),
    is_valid BOOLEAN DEFAULT TRUE,
    financial_operation VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_title_associate_doc ON title(associate_id, document_number);
CREATE INDEX idx_title_associate ON title(associate_id);

-- Parcelas
CREATE TABLE installment (
    id UUID PRIMARY KEY,
    title_id UUID NOT NULL REFERENCES title(id) ON DELETE CASCADE,
    associate_id UUID NOT NULL REFERENCES associate(id_client_esolution),
    installment_number INTEGER NOT NULL,
    due_date DATE NOT NULL,
    value DECIMAL(10,2) NOT NULL,
    paid_value DECIMAL(10,2) DEFAULT 0,
    paid_date DATE,
    is_paid BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_installment_title_num ON installment(title_id, installment_number);
CREATE INDEX idx_installment_associate ON installment(associate_id);
CREATE INDEX idx_installment_due ON installment(due_date);

-- Log de distribuicao de carteiras
CREATE TABLE distribution_log (
    id UUID PRIMARY KEY,
    workstation_id UUID NOT NULL REFERENCES workstation(id),
    associate_id UUID NOT NULL REFERENCES associate(id_client_esolution),
    distributed_by UUID NOT NULL,
    distribution_type VARCHAR(20) NOT NULL,
    distributed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_distribution_log_associate ON distribution_log(associate_id);
CREATE INDEX idx_distribution_log_workstation ON distribution_log(workstation_id);

-- Negociacoes (imutavel - apenas insert)
CREATE TABLE negotiation (
    id UUID PRIMARY KEY,
    associate_id UUID NOT NULL REFERENCES associate(id_client_esolution),
    workstation_id UUID REFERENCES workstation(id),
    negotiation_type VARCHAR(50) NOT NULL,
    value DECIMAL(12,2),
    payment_method VARCHAR(50),
    installment_count INTEGER,
    description TEXT,
    negotiated_by UUID NOT NULL,
    negotiated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_negotiation_associate ON negotiation(associate_id);
CREATE INDEX idx_negotiation_date ON negotiation(negotiated_at);
```

---

## 5. Auditoria (simple_history)

Todas as entidades principais terao auditoria via simple_history ou equivalente Rust:

```sql
-- Tabela de auditoria generica
CREATE TABLE audit_log (
    id UUID PRIMARY KEY,
    table_name VARCHAR(100) NOT NULL,
    record_id UUID NOT NULL,
    action VARCHAR(10) NOT NULL, -- INSERT, UPDATE, DELETE
    old_data JSONB,
    new_data JSONB,
    user_id UUID,
    changed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_audit_log_table_record ON audit_log(table_name, record_id);
CREATE INDEX idx_audit_log_changed ON audit_log(changed_at);
```

---

## 6. Observacoes Importantes

1. **id_client_esolution**: Esta e a chave canonica do associado. NUNCA renomear para `client_id`, `filter_id` ou similar no dominio Rust.

2. **UUIDs**: Todas as entidades principais usam UUID como PK para evitar conflitos em ambientes distribuidos.

3. **Nullabilidade**: CPF e opcional em associate/lead mas indexado. Email e obrigatorio em lead.

4. **Imutabilidade**: Tabelas de log (distribution_log, negotiation) nao permitem UPDATE - apenas INSERT.

5. **Indices**: Criados estrategicamente para queries comuns (por associate, por data, por status).

6. **Foreign Keys**: Todas as FKs tem `ON DELETE` explicito para evitar orphan records.

---

## 7. Proximos Passos

- [ ] Executar migracoes em ambiente de desenvolvimento
- [ ] Validar integridade referencial
- [ ] Testar performance dos indices
- [ ] Implementar modelo Rust com sqlx/sea-orm
- [ ] Configurar sync do rust-sync-service para novo schema

---

## 8. Historico de Versoes

| Versao | Data | Autor | Descricao |
|--------|------|-------|-----------|
| 1.0 | 2026-02-26 | Ralph | Schema inicial unificado |
