export default {
  meta: {
    type: "problem",
    docs: {
      description: "Disallow arbitrary pixel values in JSX className",
    },
    schema: [],
    messages: {
      noArbitraryPxClassname:
        "Avoid arbitrary pixel values in className. Prefer Tailwind scale tokens.",
    },
  },
  create(context) {
    return {
      "JSXAttribute[name.name='className'] > Literal[value=/\\[[0-9]+px\\]/]"(
        node,
      ) {
        context.report({ node, messageId: "noArbitraryPxClassname" });
      },
      "JSXAttribute[name.name='className'] JSXExpressionContainer > TemplateLiteral > TemplateElement[value.raw=/\\[[0-9]+px\\]/]"(
        node,
      ) {
        context.report({ node, messageId: "noArbitraryPxClassname" });
      },
    };
  },
};
