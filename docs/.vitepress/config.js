export default {
  title: "Solana Opcode Guide",
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
    ],
  },
};
