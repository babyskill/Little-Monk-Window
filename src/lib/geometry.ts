export type Rect = {
  x: number;
  y: number;
  width: number;
  height: number;
};

export function containsPoint(rect: Rect, x: number, y: number) {
  return x >= rect.x && y >= rect.y && x <= rect.x + rect.width && y <= rect.y + rect.height;
}
