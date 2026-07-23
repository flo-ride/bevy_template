---
type: Debugging Guide
title: Personna Debugging
description: Documentation on fixing asset loading issues for the Personna feature.
timestamp: "2026-07-23T18:14:50Z"
---

# Schema
- Asset: `PersonnaConfig`
- File: `assets/personna.ron`
- Resource: `PersonnaHandle`

# Context
We encountered an issue where `Assets<PersonnaConfig>` remained empty, despite the file existing. The resolution involved:
1. Ensuring the `Handle` is stored as a `Resource` (`PersonnaHandle`) to prevent it from being dropped.
2. Initializing this resource using `FromWorld` in the plugin.
3. Checking the `load_state` via `AssetServer` to ensure the asset is actually loaded before trying to access it in `Assets`.
4. Using `info!` logs to debug the `load_state` and ensure the system logic runs only when ready.

# Citations
- [Bevy Asset Server documentation](https://bevyengine.org/learn/book/getting-started/assets/)
