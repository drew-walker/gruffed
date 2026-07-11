export type NavItem = {
  title: string;
  href: string;
  summary: string;
  content: string;
};

export type NavGroup = {
  title: string;
  items: NavItem[];
};

export const navGroups: NavGroup[] = [
  {
    title: "Start",
    items: [
      {
        title: "CLI",
        href: "/docs/cli",
        summary: "Run gruffed from the terminal and CI.",
        content: "CLI command flags config root format json terminal color no color examples.",
      },
      {
        title: "Config",
        href: "/docs/config",
        summary: "JSONC schema, defaults, and rule settings.",
        content:
          "gruffed jsonc root entrypoints exclude extensions rules off error warning options.",
      },
    ],
  },
  {
    title: "Rules",
    items: [
      {
        title: "no-cycles",
        href: "/docs/rules/no-cycles",
        summary: "Detect circular module dependencies.",
        content: "circular dependency strongly connected components Tarjan cycle imports.",
      },
      {
        title: "no-unresolved",
        href: "/docs/rules/no-unresolved",
        summary: "Report imports that do not resolve to files.",
        content: "unresolved import missing file specifier parse warning source line.",
      },
      {
        title: "no-long-chains",
        href: "/docs/rules/no-long-chains",
        summary: "Flag imports past a maximum depth.",
        content: "long chain max depth entrypoint BFS imports warning architecture boundaries.",
      },
    ],
  },
  {
    title: "APIs",
    items: [
      {
        title: "Rust",
        href: "/docs/api/rust",
        summary: "Crate responsibilities and core Rust entrypoints.",
        content: "Rust crates graph builder analyzer report config console reporter Node bindings.",
      },
      {
        title: "Node",
        href: "/docs/api/node",
        summary: "napi exports and TypeScript usage.",
        content:
          "Node API buildModuleGraph analyzeGraph renderReport freeGraph graphHandle warnings.",
      },
      {
        title: "Architecture",
        href: "/docs/architecture",
        summary: "Build, analyze, report separation.",
        content: "architecture core builder analyzer reporter immutable graph separation concerns.",
      },
    ],
  },
];

export const searchItems = navGroups.flatMap((group) =>
  group.items.map((item) => ({
    ...item,
    group: group.title,
  })),
);
