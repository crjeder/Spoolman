## MODIFIED Requirements

### Requirement: Derive weight metrics
The system SHALL derive and return weight metrics in spool responses. Metrics are computed from stored fields, never stored redundantly. net_weight is read from the spool, not from the filament.

#### Scenario: Used weight derived
- **WHEN** a spool is retrieved
- **THEN** used_weight = initial_weight - current_weight is included in the response

#### Scenario: Remaining filament derived when net_weight known
- **WHEN** a spool is retrieved and the spool has a net_weight
- **THEN** remaining_filament = spool.net_weight - used_weight is included in the response

#### Scenario: No remaining_filament when net_weight absent
- **WHEN** a spool is retrieved and spool.net_weight is None
- **THEN** remaining_filament is omitted (None) from the response

## REMOVED Requirements

### Requirement: Remaining percentage derived
**Reason**: `remaining_pct` is redundant and only meaningful when net_weight is set, causing confusing "unknown" display states. The percentage is trivially computable from `remaining_filament / net_weight` by any consumer that needs it.
**Migration**: Consumers that need a percentage MUST compute it client-side: `remaining_filament / net_weight * 100`.
