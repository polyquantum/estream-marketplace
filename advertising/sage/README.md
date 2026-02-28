# estream-sage

AI agent framework for eStream — conversational chatbot/voicebot with ESLM inference, external API bridge, content generation, and SEO optimization.

## Overview

`estream-sage` is a generic AI agent platform built on Stratum and Cortex. It powers conversational AI (text and voice), bridges external APIs via MPC-garbled circuits for privacy-preserving inference, and generates SEO-optimized content from conversation data. Sage is a platform product — domain-agnostic by design, composable with any vertical (e.g. veteran services via `estream-crm`).

## Circuits

| Circuit | File | Description |
|---------|------|-------------|
| `sage_agent` | `circuits/sage_agent.fl` | Core conversational AI — intent classification, ESLM inference, RLHF feedback, external API bridge |
| `sage_content` | `circuits/sage_content.fl` | SEO content generation — topic clustering, content scoring, graph-linked knowledge base |
| `sage_voice` | `circuits/sage_voice.fl` | Voice pipeline — session management, audio processing, latency monitoring |

## Stratum Usage

- **KV storage** — session lookup for conversations, content nodes, voice sessions
- **Graph storage** — content relationship graph with topic clustering (CSR on BRAM, 50K capacity)
- **Series** — 90d retention for conversations, 365d for content, 30d for voice sessions
- **Streams** — event streams with consumers for StreamSight, Cortex, analytics, alerting

## Install

```
estream marketplace install estream-sage
```

## Cortex AI Integration

- **Conversational model** — `sage_conversational` infers responses from message history, topic, and user profile; retrains on drift (threshold 0.08)
- **SEO optimizer** — `sage_seo_optimizer` scores content and suggests keywords from engagement metrics; retrains on drift
- **Intent classifier** — inline `li_classify` for real-time intent detection on every message
- **RLHF feedback** — `sage_rate_response` collects user ratings for continuous model improvement

## External API Bridge

External model queries use an MPC-garbled circuit pattern — the external provider never sees raw user input. Request and response hashes are logged for auditability without exposing content.

## Security

- ML-DSA-87 signatures on all mutations
- ML-KEM-1024 key encapsulation
- PoVC attestation on agent responses and content nodes
- PII obfuscation/redaction via Cortex governance (user_id obfuscated, messages redacted for analytics)
- StreamSight anomaly detection on every circuit
- MPC-garbled external API bridge for privacy-preserving inference

## Dependencies

- `estream-crm ^0.1.0` — user profile and relationship data for personalized inference
