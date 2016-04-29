
export function isExpression(v:string): boolean {
  return v.length >= 2 && v[0] === "'" && v[v.length - 1] === "'";
}