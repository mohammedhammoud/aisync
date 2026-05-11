export default {
  meta: {
    type: "problem",
    docs: {
      description: "Disallow wildcard exports",
    },
    schema: [],
    messages: {
      noWildcardExports: "Wildcard exports are not allowed",
    },
  },
  create(context) {
    return {
      ExportAllDeclaration(node) {
        context.report({
          node,
          messageId: "noWildcardExports",
        });
      },
    };
  },
};
