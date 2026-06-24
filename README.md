# Little Monk Window

Little Monk Window là ứng dụng pet desktop cho macOS và Windows, hiển thị chú tiểu nhỏ cùng bong bóng câu trích, hỗ trợ nhắc chuông chánh niệm, giờ yên lặng, âm thanh tùy chỉnh và cửa sổ cài đặt kiểu macOS.

## Ảnh chụp màn hình

![Giao diện Little Monk Window](screenshot.png)

## Tính năng chính

- Chú tiểu nổi trên desktop, có thể bật/tắt hiển thị.
- Bong bóng câu trích tự động cập nhật theo ngôn ngữ đã chọn.
- Hỗ trợ chuông chánh niệm, tiếng gõ mõ và âm thanh ngoài.
- Có giờ yên lặng để tạm ngưng nhắc nhở vào khung giờ mong muốn.
- Cửa sổ Settings hỗ trợ tiếng Việt và nhiều tuỳ chỉnh nhanh.
- Tray/menu bar với icon riêng của ứng dụng.

## Tải về

- Bản phát hành mới nhất: [Releases](https://github.com/babyskill/Little-Monk-Window/releases)
- File cài đặt Windows nằm trong từng release dưới dạng `.exe` hoặc `.msi`.

## Chạy dự án

```bash
npm install
npm run check
npm run build
cd src-tauri && cargo check
```

## Phát triển

- Frontend: Svelte + Vite
- Desktop shell: Tauri v2
- Ngôn ngữ chính: Rust

## Cấu trúc dự án

- `src/`: giao diện và logic frontend
- `src-tauri/`: cấu hình và logic Tauri/Rust
- `screenshot.png`: ảnh minh họa dùng trong README

## Ghi chú

- Dự án macOS gốc tại `/Users/appdexter/Dev/Little-Monk-Mac` là nguồn tham chiếu hành vi và icon.
- Nếu bạn chỉnh giao diện hoặc luồng cài đặt, nên cập nhật lại ảnh chụp màn hình trong README để đồng bộ.
