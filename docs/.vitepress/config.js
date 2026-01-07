export default {
  title: "Solana Opcode Guide",
  description:
    "Courtesy of Distributed Atomic State Machine Algorithms Corporation (DASMAC)",
  head: [
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
  ],
  srcDir: "src",
  markdown: {
    lineNumbers: true,
    math: true,
  },
  themeConfig: {
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
        ],
      },
      { text: "Resources", link: "/resources" },
      { text: "Opcodes", link: "/opcodes" },
    ],
  },
};
