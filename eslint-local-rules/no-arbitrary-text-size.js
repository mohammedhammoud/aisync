export default {
  meta: {
    type: "problem",
    docs: {
      description: "Disallow arbitrary text sizes in JSX className",
    },
    schema: [],
    messages: {
      noArbitraryTextSize:
        "Avoid arbitrary text sizes. Use standard Tailwind sizes like text-xs/text-sm/text-base.",
    },
  },
  create(context) {
    return {
      "JSXAttribute[name.name='className'] > Literal[value=/text-\\[[0-9]+px\\]/]"(
        node,
      ) {
        context.report({ node, messageId: "noArbitraryTextSize" });
      },
      "JSXAttribute[name.name='className'] JSXExpressionContainer > TemplateLiteral > TemplateElement[value.raw=/text-\\[[0-9]+px\\]/]"(
        node,
      ) {
        context.report({ node, messageId: "noArbitraryTextSize" });
      },
    };
  },
};
