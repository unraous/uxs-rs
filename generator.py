from PIL import Image
import os

# 打开 2048x2048 的 PNG
img = Image.open('uxs.png')  # 你的 2048x2048 PNG

# 确保输出目录存在
os.makedirs('src-tauri/icons', exist_ok=True)

# 生成 PNG 版本（不同尺寸）
sizes = [
    (32, 'icons/32x32.png'),
    (128, 'icons/128x128.png'),
    (256, 'icons/128x128@2x.png'),  # 高DPI版本
    (512, 'icons/512x512.png'),     # 可选，某些系统需要
]

for size, filename in sizes:
    resized = img.resize((size, size), Image.Resampling.LANCZOS)
    resized.save(f'src-tauri/{filename}')
    print(f'已保存 {filename}')

# 生成 ICO（Windows）
ico_img = img.resize((256, 256), Image.Resampling.LANCZOS)
ico_img.save('src-tauri/icons/icon.ico')
print('已保存 icon.ico')

# 如果要支持 macOS，需要生成 ICNS（需要额外工具）
# 可用在线工具：https://icoconvert.com/ 或 https://cloudconvert.com/
# 或使用命令行：sips -s format icns icon.png -o icon.icns
print("\nmacOS ICNS: 使用在线工具转换 512x512.png 为 icon.icns")