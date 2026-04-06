from PIL import Image

Image.open('uxs.png').save(
    'src-tauri/icons/icon.ico',
    format='ICO'
)
print('已保存 icon.ico')