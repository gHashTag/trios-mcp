# Rule 06 — Language Policy

## Public artefacts: English-only

The following are **English-only** at the time of writing:

- `README.md` (with the documented bilingual exception below)
- `AGENTS.md`, `CLAUDE.md`, `AGENT_WAKEUP.md`
- `docs/agent-rules/*.md`
- Tool descriptions and error messages emitted by `trios-mcp` itself
- Public commit messages, PR titles and bodies, release notes,
  CHANGELOG entries

## Documented bilingual exception

The block titled
"**TRIOS PhD — canonical context (RU + EN)**" at the top of `README.md`
is the only scoped exception. It exists because the canonical TRIOS PhD
context is established bilingually in the broader project ecosystem.
The RU portion is a translation of the EN portion, not new content.

Do not extend this exception to other sections, other files, or other
languages without a separate explicit decision.

## Maintainer chat

Chat with the maintainer (issues, discussions, in-session prompts) may
be Russian. Agents may respond in Russian when the user writes in
Russian. The translation back to English happens at artefact-write
time, not at read time.

## Implication for agents

Before committing or opening a PR, do a quick scan for Cyrillic
characters in changed files outside the bilingual block. If any
appear, either move them into the bilingual block (with a justified
reason) or translate them.
