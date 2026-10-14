import {
  defineConfig,
  singleIdVerification,
  tableRequirements,
} from "@lapismd/spec-validator";

export default defineConfig(tableRequirements(), {
  name: "jj-acp",
  idPattern: /^JAC-[A-Z]+-\d{3}$/,
  tableSection: "Requirements",
  ruleIds: {
    summary: "JAC-GOV-001",
    governance: "JAC-GOV-001",
    verification: "JAC-GOV-001",
    book: "JAC-GOV-001",
    bookIgnore: "JAC-GOV-001",
    specFirst: "JAC-GOV-002",
    internal: "JAC-GOV-001",
  },
  validators: {
    summary: true,
    governance: {
      extras: ["AGENTS.md", "README.md"],
      normative: true,
      proseLimits: false,
      acceptance: false,
      references: true,
      changeMap: true,
    },
    verification: singleIdVerification({
      headers: {
        ids: ["ID"],
        status: ["Status"],
        evidence: ["Evidence"],
        required: [],
      },
      statuses: ["Implemented", "Planned"],
    }),
    book: true,
    specFirst: {
      mode: "mapped",
      canonicalPattern: "^spec/src/(?!SUMMARY|verification).+\\.md$",
      ignore: [
        "(^|/)node_modules/",
        "(^|/)(?:target|dist|build)/",
        "^spec/book/",
        "^spec/src/(?:SUMMARY|verification)\\.md$",
      ],
      rules: [
        {
          pattern: "^\\.github/",
          chapters: ["spec/src/spec-governance.md", "spec/src/release.md"],
        },
        {
          pattern: "^crates/(?:[a-z0-9]+-)*acp-protocol/",
          chapters: ["spec/src/protocol.md"],
        },
        {
          pattern: "^crates/(?:[a-z0-9]+-)*acp-agent/",
          chapters: ["spec/src/agents.md"],
        },
        {
          pattern: "^crates/(?:[a-z0-9]+-)*acp-runner/",
          chapters: ["spec/src/runner.md"],
        },
        {
          pattern: "^crates/(?:[a-z0-9]+-)*acp-control/",
          chapters: ["spec/src/control.md"],
        },
        {
          pattern: "^crates/(?:[a-z0-9]+-)*acp-cli/",
          chapters: ["spec/src/cli.md"],
        },
        {
          pattern: "^crates/(?:[a-z0-9]+-)*acp-testkit/",
          chapters: ["spec/src/compatibility.md"],
        },
        {
          pattern: "^(?:tests/|docs/)",
          chapters: ["spec/src/compatibility.md"],
        },
        {
          pattern:
            "^(?:scripts/|AGENTS\\.md$|spec-validator\\.config\\.mjs$|spec/book\\.toml$|\\.gitignore$|\\.prettier)",
          chapters: ["spec/src/spec-governance.md"],
        },
        {
          pattern: "^rustfmt\\.toml$",
          chapters: ["spec/src/spec-governance.md"],
        },
        {
          pattern:
            "^(?:Cargo\\.(?:toml|lock)$|rust-toolchain\\.toml$|package\\.json$|pnpm-lock\\.yaml$|README\\.md$|CHANGELOG\\.md$|CONTRIBUTING\\.md$|SECURITY\\.md$|\\.gitattributes$|LICENSE\\.md$)",
          chapters: ["spec/src/architecture.md"],
        },
      ],
      protected: [
        "^(?:\\.github/|crates/|tests/|scripts/|docs/|Cargo\\.(?:toml|lock)$|rust-toolchain\\.toml$|rustfmt\\.toml$|package\\.json$|pnpm-lock\\.yaml$|README\\.md$|CHANGELOG\\.md$|CONTRIBUTING\\.md$|SECURITY\\.md$|\\.gitattributes$|LICENSE\\.md$|AGENTS\\.md$|spec-validator\\.config\\.mjs$|spec/book\\.toml$|\\.gitignore$|\\.prettier)",
      ],
    },
  },
  check: {
    lanes: [
      {
        name: "rustfmt",
        command: "cargo",
        args: ["fmt", "--all", "--", "--check"],
      },
      {
        name: "prettier",
        command: "pnpm",
        args: ["exec", "prettier", "--check", "."],
      },
      {
        name: "boundaries",
        command: "node",
        args: ["scripts/check-boundaries.mjs"],
      },
      {
        name: "architecture",
        command: "node",
        args: ["scripts/rust-architecture.mjs"],
      },
      {
        name: "progress",
        command: "node",
        args: ["scripts/check-progress.mjs"],
      },
      {
        name: "upstream-pin",
        command: "node",
        args: ["scripts/upstream-sync.mjs", "--self-test"],
      },
      {
        name: "governance-tests",
        command: "node",
        args: [
          "--test",
          "scripts/governance.test.mjs",
          "scripts/rust-architecture.test.mjs",
          "scripts/upstream-lock.test.mjs",
          "scripts/progress-vcs.test.mjs",
          "scripts/release.test.mjs",
          "scripts/registry-check.test.mjs",
        ],
      },
      {
        name: "rust-tests",
        command: "cargo",
        args: ["test", "--workspace", "--locked"],
      },
    ],
    build: true,
    first: true,
  },
});
