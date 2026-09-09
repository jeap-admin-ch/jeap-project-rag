# Agent notes for jeap-project-rag

See `CLAUDE.md` for the full project architecture and development guide. This file holds
conventions that apply across coding agents, not just Claude.

## Keep README.md short - detailed docs go in docs/

`README.md` is the front door: title, a short overview, the feature list, prerequisites,
installation, and a minimal quickstart, plus a "Documentation" section linking out to everything
else. It is not the place for exhaustive tool references, architecture diagrams, configuration
tables, benchmarks, or a full troubleshooting guide - those live under `docs/` as their own pages
(see the "Documentation" section in `README.md` for the current list), each linked from the README
and cross-linked to each other where relevant. When adding a substantial new section to the README,
ask whether it's actually onboarding material or reference material - reference material belongs in
`docs/`, with only a link and a one-line summary left in the README.

## Documentation must be valid MDX

`README.md` and everything under `docs/` gets published on the `jeap-admin-ch.github.io`
Docusaurus site, which compiles Markdown as MDX. Content that's valid GitHub-flavored Markdown can
still fail MDX compilation and break that site's build. In particular:

- **Never write a bare `<` immediately followed by a digit or letter** in prose — e.g. `<5 minutes`
  or `<500MB`. MDX parses `<` as the start of a JSX tag and fails with "Unexpected character ... expected
  a character that can start a name" the moment what follows isn't a valid tag-name character.
  Write it out instead: "under 5 minutes", "under 500MB". `>` does not have this problem.
- Fenced code blocks (`` ``` ``) are opaque to the MDX parser, so `<`/`{`/`}` inside them (JSON
  examples, shell snippets, etc.) are safe and don't need escaping.
- Outside of fenced code, a stray `{` starting a line is also parsed as a JSX expression container
  and can fail the same way.

Before publishing a documentation change here, skim it for these patterns. There is no local way
to run the actual MDX compiler from this repo — the failure only surfaces in
`jeap-admin-ch.github.io`'s build (or its `OpenSourcePreconditionEnforcer`-equivalent check, if one
is later wired up here too).
