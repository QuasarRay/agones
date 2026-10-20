# AGENTS.MD

## Project Goals:

1. Reimplement Agones in Rust.

2. Rusternetes Integration and Compatibility.

3. Reimplement the Original Agones Tests with Kani, Bolero, CGP-Oriented metaprogramming, and metaprogramming with proc macro 2, proc macro 2 attribute, and proc macro 2 derive and proc macro 2 delegate, Egglog, and Genco while the Test invariants remain Identical. Bolero's primary feature will be property testing, to ensure that the tests are run against an invariant not the design. The tests must not accept the authoritative reference specification of the project to be correct. Which is why, Boleros property tests will be used to test both authoritative reference specification, and implementation against invariants. Bolero will use Kani as its backend, to ensure that the tests do not merely tests against samples, but ensure that the project is symbolically tested against every possible scenario to satisfy the test invariant, using Kani's formal verification backend.

4. AGGRESSIVELY use Metaprogramming and Automation to accelerate and simplify development. AGGRESSIVELY use CGP-Oriented metaprogramming, and metaprogramming with proc macro 2, proc macro 2 attribute, and proc macro 2 derive and proc macro 2 delegate to Automate, simplify and accelerate development

6. Use CGP and derive-deftly and comprende AGGRESSIVELY, to accelerate development and reduce boilerplate: https://github.com/contextgeneric/cgp.git, https://gitlab.torproject.org/Diziet/rust-derive-deftly, https://github.com/QuasarRay/comprende.git

7. This AGENTS.MD file must exist in every directory of this project

8. Do not modify main branch. always work in agones-rs-dev branch

9. Make stackable pull requests incrementally to ensure that no work is duplicated or lost. turn my github account into your workspace. merge into agones-rs-dev after each milestone.

10. Cluster Orchestrator independence must be an emergent property of the Architecture of agones-rs. Therefore, the design philosophy and paradigm of agones-rs must be CGP Oriented design, which means, CGP must own manage the entire application lifecycle. Therefore, despite the fact that it is forbidden for AI to make any attempts to create a plugin system, and it is also forbidden to implement integration with any Cluster Orchestrator except Rusternetes, which is the only permitted integration, Cluster Orchestrator independence must be an emergent property of the Architecture of agones-rs. In other words, agones-rs must be unintentionally interoperable with multiple Cluster Orchestrators seamlessly, despite strict prohibition of integrating agones-rs with any Cluster Orchestrator with Rusternetes being the ONLY exception.


Target Project Repo: https://github.com/QuasarRay/agones.git
