# ADR-001: Arquitetura Modular Rust - Hexagonal (Ports and Adapters)

**Status:** Aprovado
**Data:** 2026-02-26
**Projeto:** Unificacao CRM2 + COB2 em Rust + Dioxus

---

## Contexto

O sistema legado consiste em dois aplicativos Django separados (CRM2 e COB2) que compartilham entidades, mas possuem camadas de dominio duplicadas e acoplamento direto ao banco de dados SQL Server. A migracao para Rust requer uma arquitetura que:

- Elimine duplicacao de codigo entre dominios
- Permita evolucao independente dos modulos
- Facilite testes unitarios sem dependencia de banco de dados
- Suporte migracao incremental (por ondas) sem big bang

---

## Decisao

Adotar **Arquitetura Hexagonal (Ports and Adapters)** com **Workspace Rust modular** dividido em 4 crates:

```
apps/backend/
├── Cargo.toml          # Workspace root
└── src/
    ├── shared/         # Nucleo compartilhado (kernel)
    ├── crm/            # Bounded Context CRM
    ├── cobranca/       # Bounded Context Cobranca
    └── integration/    # Adaptadores de sincronizacao com legado
```

### Estrutura interna de cada modulo

Cada crate segue a mesma estrutura de 3 camadas:

```
<modulo>/
├── domain/
│   ├── entities/       # Entidades de dominio (sem dependencias externas)
│   ├── ports/          # Traits (interfaces) de repositorios
│   └── value_objects/  # Tipos de valor imutaveis
├── application/
│   ├── ports/          # Casos de uso (interfaces de entrada)
│   └── services/       # Orquestracao da logica de dominio
└── infrastructure/
    └── persistence/    # Implementacoes de repositorios (PostgreSQL)
```

### Contratos entre camadas

```
infrastructure -> application -> domain
     |               |             |
 (implementa)   (orquestra)   (define regras)
```

- **Domain** nao depende de nenhuma outra camada
- **Application** depende apenas de Domain (via traits/ports)
- **Infrastructure** implementa as interfaces definidas em Domain

---

## Dependencias entre modulos

```
integration -> crm, cobranca, shared
crm         -> shared
cobranca    -> shared
shared      -> (nenhum modulo interno)
```

---

## Consequencias

### Positivas

1. **Testabilidade**: Dominio pode ser testado sem banco de dados (mock dos repositorios)
2. **Independencia de framework**: Troca de banco de dados nao afeta logica de dominio
3. **Evolucao modular**: Cada bounded context pode evoluir independentemente
4. **Migracao incremental**: Novos dominios podem ser adicionados sem alterar os existentes
5. **Clareza de responsabilidades**: Cada camada tem responsabilidade bem definida

### Negativas / Trade-offs

1. **Boilerplate inicial**: Mais arquivos para funcionalidades simples
2. **Indirection**: Chamadas atravessam mais camadas (ports -> adapters)
3. **Complexidade para novos contribuidores**: Curva de aprendizado da arquitetura hexagonal

---

## Alternativas consideradas

### Monolito simples (rejeitado)
Toda a logica em um unico crate. Rejeitado porque impede evolucao independente dos dominios e replica o problema dos legados.

### Microservicos (rejeitado)
Um servico por dominio. Rejeitado para esta fase por adicionar complexidade de rede e deployment desnecessaria antes de validar o modelo de dados unificado.

### Clean Architecture com camadas nomeadas (considerado)
Muito similar ao Hexagonal. Optou-se por Hexagonal por ter terminologia mais precisa para Ports e Adapters, facilitando comunicacao entre equipes.

---

## Referencias

- Livro: "Domain-Driven Design" - Eric Evans
- Artigo: "Hexagonal Architecture" - Alistair Cockburn
- Documentacao: `apps/backend/AGENTS.md`
- Schema canonico: `docs/01-prd/SCHEMA_CANONICO_UNIFICADO.md`
