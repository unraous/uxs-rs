from PIL import Image
import numpy as np

width, height = 2560, 1600

# 生成高斯黑白噪声
rng = np.random.default_rng(seed=42)
noise = rng.normal(128, 30, (height, width))  # 均值128，标准差30
noise = np.clip(noise, 0, 255)  # 限制在0-255范围内

# 转为图片
img = Image.fromarray(noise.astype(np.uint8), mode='L')
img.save('noise_texture.png')
print("高斯黑白噪声纹理已生成: noise_texture.png")