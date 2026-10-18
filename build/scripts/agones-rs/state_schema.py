"""The small ledger schema; uses the host's bundled Pydantic implementation."""
from typing import Any, Literal

from pydantic import BaseModel, ConfigDict, Field, StrictInt

STAGES = ["DISCOVERED", "IMPLEMENTING", "IMPLEMENTED", "EXTRACTING", "PROPERTIES_READY", "VERIFYING", "VERIFIED", "DELIVERED"]
TRANSITIONS = {stage: ([STAGES[i+1]] if i < len(STAGES)-1 else []) + (["IMPLEMENTING"] if i > 1 else [])
               for i, stage in enumerate(STAGES)}


class WorkItem(BaseModel):
    model_config = ConfigDict(extra="forbid")
    id: str
    requirement_ids: list[str]
    feature_ids: list[str]
    test_ids: list[str]
    dependencies: list[str]
    owned_paths: list[str]
    stage: Literal["DISCOVERED", "IMPLEMENTING", "IMPLEMENTED", "EXTRACTING", "PROPERTIES_READY", "VERIFYING", "VERIFIED", "DELIVERED"]
    disposition: Literal["READY", "BLOCKED", "UNSUPPORTED", "STALE", "SUPERSEDED", "EXCLUDED_BY_USER"]
    implementation_gate: str | None
    source_config_binding: dict[str, Any]
    evidence_refs: list[str]
    next_action: str


class State(BaseModel):
    model_config = ConfigDict(extra="forbid", strict=True, json_schema_extra={
        "x-legal-stage-transitions": TRANSITIONS,
        "x-revision-rule": "Increment by one; preserve predecessor revision/hash and immutable transition receipt. IMPLEMENTED needs a current gate; EXTRACTING requires that gate first.",
        "x-schema-migration-rule": "Explicit versioned migrations preserve prior schemas, state identities, authorization and affected-evidence invalidation. No implicit migration.",
    })
    schema_version: Literal[1]
    revision: StrictInt = Field(ge=1)
    previous_revision: StrictInt | None
    previous_sha256: str | None
    contract_sha256: str = Field(pattern=r"^[a-f0-9]{64}$")
    baseline_id: str = Field(pattern=r"^[a-f0-9]{40}$")
    scope_id: str
    workspace: dict[str, Any]
    goal: dict[str, Any]
    migration_status: Literal["INCOMPLETE", "COMPLETE"]
    work_items: list[WorkItem]
    jobs: list[dict[str, Any]]
    refactoring_reviews: list[dict[str, Any]]
    architecture_matrix: list[dict[str, Any]] = Field(min_length=8, max_length=8)
    delivery: dict[str, Any]
    blockers: list[dict[str, Any]]
    artifact_policy: dict[str, Any]
