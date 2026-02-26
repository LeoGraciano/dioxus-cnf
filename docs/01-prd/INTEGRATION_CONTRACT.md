# Integration Contract - rust-sync-service

**Version**: 1.0.0
**Last Updated**: 2026-02-26
**Status**: Active

## Overview

This document defines the contract between the unified Rust+Dioxus backend and the `rust-sync-service` that synchronizes data from SQL Server to PostgreSQL.

## Event Stream

- **Redis Stream**: `sync:progress`
- **Consumer Group**: `sync_workers`
- **Redis DB**: 8

## Message Types

### 1. progress
```json
{
  "msg_type": "progress",
  "current": 100,
  "total": 144000,
  "message": "Processando associado 12345"
}
```

### 2. associate_saved
```json
{
  "msg_type": "associate_saved",
  "id_client_esolution": "12345",
  "titles": 5,
  "bills": 12,
  "installments": 36
}
```

### 3. sync_complete
```json
{
  "msg_type": "sync_complete",
  "total_associates": 144000,
  "duration_seconds": 6720,
  "errors": 0
}
```

### 4. error
```json
{
  "msg_type": "error",
  "message": "Error: connection timeout",
  "id_client_esolution": "12345"
}
```

## API Endpoints

### POST /api/sync/start

Starts a sync operation.

**Query Parameters**:
- `client_id` (optional): Filter by specific associate ID

**Response**:
```json
{
  "status": "started",
  "channel": "sync:progress",
  "stream": "sync:progress"
}
```

**Headers**:
- `Authorization`: `Bearer <SYNC_API_TOKEN>`

## Reconciliation Contract

The backend must implement reconciliation to detect divergences between:
1. Expected record count from legacy
2. Actual records in unified database
3. Sync event processing status

### Reconciliation Request
```json
{
  "entity_type": "associate",
  "batch_id": "uuid",
  "expected_count": 500,
  "sync_start_time": "2026-02-26T10:00:00Z"
}
```

### Reconciliation Response
```json
{
  "batch_id": "uuid",
  "expected_count": 500,
  "actual_count": 498,
  "divergences": [
    {
      "id_client_esolution": "12346",
      "issue": "missing"
    },
    {
      "id_client_esolution": "12347",
      "issue": "hash_mismatch"
    }
  ],
  "report_generated_at": "2026-02-26T10:05:00Z"
}
```

## Idempotent Reprocessing

To support idempotent reprocessing:

1. **Sync Event Tracking**: Each sync event has a unique ID and can be retried
2. **Upsert Pattern**: All writes use UPSERT to prevent duplicates
3. **Retry with Idempotency Key**: Use entity_id + timestamp as idempotency key
4. **State Machine**: Events go through Pending -> Processing -> Completed/Failed/Retrying

## Read Operations

Read operations on the unified system should:
- Use local PostgreSQL data (not SQL Server)
- Continue working even if sync is failing
- Have eventual consistency guarantees (data may be stale during sync)
- Use the `sync_events` table to track data freshness

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2026-02-26 | Initial contract |
