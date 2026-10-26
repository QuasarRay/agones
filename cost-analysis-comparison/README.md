# Milestone cost comparisons

No implementation milestone was executed while authoring this roadmap. This directory therefore contains instructions, not fabricated actual-cost reports.

The sole operational policy is [roadmap/alternation-policy.md](../roadmap/alternation-policy.md). Copy the following structure into `NNNN-MNNNN.md` only for a real completed milestone. Leave unavailable fields unknown; use PROVISIONAL when ending counters have not finalized.

```yaml
record_kind: milestone_cost_comparison
status: PROVISIONAL | FINAL | UNKNOWN_ATTRIBUTION
milestone_id: MNNNN
route: MANUAL | CPG
goal_id: unknown
thread_id: unknown
model_and_settings: unknown
source_snapshot: unknown
forecast_id: F-MNNNN-v1
forecast_baseline_sha256: unknown
predicted_tokens: {low: unknown, base: unknown, high: unknown}
start_observation: {timestamp: unknown, source: unknown, raw: unknown}
end_observation: {timestamp: unknown, source: unknown, raw: unknown}
actual_tokens: unknown
actual_credits: unknown
token_delta_from_base: unknown
absolute_token_difference_from_base: unknown
actual_to_base_ratio: unknown
inside_planning_range: unknown
shared_or_unattributed_cost: unknown
```

## Delivered scope

Record actual authored/verified/delivered artifacts and remaining gaps separately. Link exact source and milestone identities; do not expand a partial milestone to a whole-route claim.

## Cost calculation and explanation

Show the attributable counter segments, calculation, excluded/unattributed work, baseline comparison and concrete reason for deviation. Unknown is not zero. Preserve model changes and cache-accounting semantics without double counting.

## Future forecast revision

Record changed assumptions, affected future milestone IDs, revised ranges and their rationale. Retain the original prediction; mandatory route policy remains unchanged. Do not start a new analysis or verification campaign to fill unavailable measurements.

The historical 9,834,797-token Goal snapshot is a mixed/unattributed observation, not an actual cost for any new roadmap milestone. Import older milestone measurements only when both boundaries and attribution are documented.
