export const kebabToCamel = kebabString =>
  kebabString.replace(/-([a-z])/g, g => g[1].toUpperCase());
