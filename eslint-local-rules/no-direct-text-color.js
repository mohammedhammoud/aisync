export default {
  meta: {
    type: "problem",
    docs: {
      description:
        "Disallow direct arbitrary text color classes in JSX className",
    },
    schema: [],
    messages: {
      noDirectTextColor:
        "Avoid direct text color classes in components. Use <Text /> tones or theme tokens.",
    },
  },
  create(context) {
    return {
      "JSXAttribute[name.name='className'] > Literal[value=/text-\\[#/]"(node) {
        context.report({ node, messageId: "noDirectTextColor" });
      },
      "JSXAttribute[name.name='className'] JSXExpressionContainer > TemplateLiteral > TemplateElement[value.raw=/text-\\[#/]"(
        node,
      ) {
        context.report({ node, messageId: "noDirectTextColor" });
      },
    };
  },
};
