export default {
  title: "Solana Opcode Guide",
  description:
    "Courtesy of Distributed Atomic State Machine Algorithms Corporation (DASMAC)",
  head: [
    [
      "link",
      {
        rel: "icon",
        href: "/favicon-light.png",
        media: "(prefers-color-scheme: light)",
      },
    ],
    [
      "link",
      {
        rel: "icon",
        href: "/favicon-dark.png",
        media: "(prefers-color-scheme: dark)",
      },
    ],
    ["link", { rel: "apple-touch-icon", href: "/favicon-light.png" }],
    ["meta", { property: "og:site_name", content: "DASMAC" }],
    ["meta", { property: "og:type", content: "website" }],
    ["meta", { property: "og:url", content: "https://opcodes.dasmac.com/" }],
    ["meta", { property: "og:title", content: "Solana Opcode Guide" }],
    [
      "meta",
      {
        property: "og:description",
        content:
          "Courtesy of Distributed Atomic State Machine Algorithms Corporation (DASMAC)",
      },
    ],
    ["meta", { property: "og:image", content: "https://opcodes.dasmac.com/dasmac-banner.png" }],
    ["meta", { name: "twitter:card", content: "summary_large_image" }],
    ["meta", { name: "twitter:title", content: "Solana Opcode Guide" }],
    [
      "meta",
      {
        name: "twitter:description",
        content:
          "Courtesy of Distributed Atomic State Machine Algorithms Corporation (DASMAC)",
      },
    ],
    ["meta", { name: "twitter:image", content: "https://opcodes.dasmac.com/dasmac-banner.png" }],
  ],
  srcDir: "src",
  markdown: {
    lineNumbers: true,
    math: true,
  },
  themeConfig: {
    outline: "deep",
    editLink: {
      pattern:
        "https://github.com/DASMAC-com/solana-opcode-guide/blob/main/docs/src/:path",
      text: "Contribute to this page",
    },
    sidebar: [
      { text: "Welcome", link: "/" },
      { text: "Quickstart", link: "/quickstart" },
      {
        collapsed: false,
        text: "Examples",
        link: "/examples/",
        items: [
          { text: "Memo", link: "/examples/memo" },
          { text: "Fibonacci", link: "/examples/fibonacci" },
          { text: "Transfer", link: "/examples/transfer" },
          { text: "Counter", link: "/examples/counter" },
          { text: "Tree", link: "/examples/tree" },
        ],
      },
      {
        collapsed: false,
        text: "Indices",
        link: "/indices/",
        items: [
          { text: "Opcodes", link: "/indices/opcodes" },
          { text: "Syscalls", link: "/indices/syscalls" },
          { text: "Resources", link: "/indices/resources" },
        ],
      },
    ],
  },
};
