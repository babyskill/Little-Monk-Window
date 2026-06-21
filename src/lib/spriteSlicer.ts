export type SliceRect = { x: number; y: number; width: number; height: number };

export function sliceAlphaGrid(imageData: ImageData, alphaThreshold = 16): SliceRect[] {
  const { width, height, data } = imageData;
  const rows: boolean[] = new Array(height).fill(false);

  for (let y = 0; y < height; y += 1) {
    const offset = y * width * 4;
    for (let x = 0; x < width; x += 1) {
      if (data[offset + x * 4 + 3] > alphaThreshold) {
        rows[y] = true;
        break;
      }
    }
  }

  const rowBands = segments(rows);
  const rects: SliceRect[] = [];

  for (const row of rowBands) {
    const cols: boolean[] = new Array(width).fill(false);
    for (let y = row.start; y < row.end; y += 1) {
      const offset = y * width * 4;
      for (let x = 0; x < width; x += 1) {
        if (data[offset + x * 4 + 3] > alphaThreshold) {
          cols[x] = true;
        }
      }
    }

    for (const col of segments(cols)) {
      rects.push({
        x: col.start,
        y: row.start,
        width: col.end - col.start,
        height: row.end - row.start,
      });
    }
  }

  return rects;
}

function segments(values: boolean[]): Array<{ start: number; end: number }> {
  const result: Array<{ start: number; end: number }> = [];
  let start: number | null = null;

  for (let i = 0; i < values.length; i += 1) {
    if (values[i] && start === null) {
      start = i;
    } else if (!values[i] && start !== null) {
      result.push({ start, end: i });
      start = null;
    }
  }

  if (start !== null) {
    result.push({ start, end: values.length });
  }

  return result;
}
