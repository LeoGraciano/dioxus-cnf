// CRM HTTP Handlers
// Each sub-module exposes request/response types and handler functions
// that delegate to application use-cases.
//
// Endpoints covered:
//   POST   /crm/leads                                - criar lead
//   GET    /crm/leads                                - listar leads
//   GET    /crm/leads/{id}                           - obter lead
//   PUT    /crm/leads/{id}/status                    - atualizar status
//   GET    /crm/leads/{id}/history                   - histórico auditável
//   GET    /crm/pipeline                             - estágios do pipeline
//   POST   /crm/scheduling                           - agendar
//   PUT    /crm/scheduling/{id}/status               - atualizar status agendamento
//   POST   /crm/checkin                              - registrar check-in
//   POST   /crm/checkin/{id}/checkout                - registrar check-out
//   POST   /crm/workshops                            - criar workshop
//   PUT    /crm/workshops/{id}/status                - atualizar status workshop
//   POST   /crm/sales                                - tentativa de venda
//   POST   /crm/contracts                            - criar contrato
//   PUT    /crm/contracts/{id}/status                - atualizar status contrato
//   POST   /crm/bracelets                            - emitir pulseira

pub mod bracelet;
pub mod checkin;
pub mod contract;
pub mod lead;
pub mod pipeline;
pub mod sale;
pub mod scheduling;
pub mod workshop;
