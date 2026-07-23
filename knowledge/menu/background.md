---
type: UI Feature
title: Main Menu Background
description: Documentation on the abstract particle/spark background animation for the main menu.
timestamp: "2026-07-23T18:33:08Z"
---

# Schema
- Component: `AbstractBackground`, `BackgroundBlob`
- Logic: `animate_abstract_background`
- Visuals: Particle-based spark simulation.

# Context
The main menu background was refactored to align with the "The Thirsty Skeleton" medieval fantasy tavern theme.
1. Replaced original "space-like" blobs with spark-like particles.
2. Implemented vertical ascension with horizontal oscillation to simulate rising embers or tavern dust.
3. Used warm orange-toned colors (`srgba(0.95, 0.6, 0.2, 0.4)`) against a dark tavern-wall background.

# Examples
The movement logic is controlled via `animate_abstract_background`:
```rust
let x = blob.base_x + (t * blob.speed * 2.0 + blob.phase).sin() * 10.0;
let y = (blob.base_y - (t * 15.0 * blob.speed)) % 100.0;
```
