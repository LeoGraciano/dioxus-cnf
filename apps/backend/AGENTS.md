# Backend Rust - Guidelines

## Architecture

This backend follows **Hexagonal Architecture** (Ports and Adapters) with a domain-driven design structure:

```
apps/backend/src/
├── crm/                    # Bounded Context - CRM
│   ├── application/        # Use cases & services
│   │   ├── ports/          # Input/Output interfaces
│   │   └── services/       # Application services
│   ├── domain/             # Domain logic
│   │   ├── entities/       # Domain entities
│   │   └── ports/          # Domain ports (traits)
│   └── infrastructure/     # External adapters
│       └── persistence/    # Database implementations
│
└── shared/                 # Shared kernel (cross-cutting)
    ├── application/
    │   ├── ports/
    │   └── services/
    ├── domain/
    │   ├── entities/       # Shared entities (organized by entity)
    │   ├── ports/
    │   └── value_objects/
    └── infrastructure/
        └── persistence/
```

## Entity Organization

Each entity should be in its own file within the appropriate `entities/` directory. Use singular names for entity files.

### Shared Entities
```
entities/
├── mod.rs
├── associate.rs    # Associate + AssociateStatus
├── city.rs         # City
├── state.rs        # State
├── user.rs         # User + UserProfile
└── workgroup.rs    # Workgroup
```

### CRM Entities
```
entities/
├── mod.rs
├── lead.rs              # Lead + LeadSource + LeadStatus
├── pipeline_stage.rs    # PipelineStage
├── sale.rs              # Sale
├── workshop.rs          # Workshop + WorkshopType + WorkshopStatus
├── contract.rs          # Contract + ContractStatus
├── bracelet.rs          # Bracelet + BraceletStatus
├── scheduling.rs        # Scheduling + SchedulingType + SchedulingStatus
└── check_in.rs          # CheckIn
```

**mod.rs exports:**
```rust
// Shared
pub mod associate;
pub mod city;
pub mod state;
pub mod user;
pub mod workgroup;

pub use associate::{Associate, AssociateStatus};
pub use city::City;
pub use state::State;
pub use user::{User, UserProfile};
pub use workgroup::Workgroup;

// CRM
pub mod lead;
pub mod pipeline_stage;
pub mod sale;
pub mod workshop;
pub mod contract;
pub mod bracelet;
pub mod scheduling;
pub mod check_in;

pub use lead::{Lead, LeadSource, LeadStatus};
pub use pipeline_stage::PipelineStage;
pub use sale::Sale;
pub use workshop::{Workshop, WorkshopType, WorkshopStatus};
pub use contract::{Contract, ContractStatus};
pub use bracelet::{Bracelet, BraceletStatus};
pub use scheduling::{Scheduling, SchedulingType, SchedulingStatus};
pub use check_in::CheckIn;
```

## Coding Conventions

- Use `pub` for public API
- Derive `Debug, Clone, Serialize, Deserialize` for all entities
- Use enums for status/role fields with `PartialEq`
- Implement `Default` for status enums when appropriate
- Use `chrono::DateTime<Utc>` for timestamps
- Use `uuid::Uuid` for unique identifiers
